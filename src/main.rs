mod cli;
mod config;
mod download;
mod metadata;
mod util;
use std::{
    fs::File,
    io::{BufReader, ErrorKind},
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::bail;
use indicatif::{ProgressBar, ProgressStyle};
use ncm_api::MusicApi;
use tokio::{
    self,
    fs::{self},
    io::AsyncWriteExt,
    sync::{Mutex, Semaphore},
};

use crate::{
    config::{Config, ConfigError},
    download::{DownloadOptions, download_file},
    metadata::{TrackInfo, write_metadata},
};

const MAX_CONS: usize = 0;
const MAX_NAME_LENGTH: usize = 200;
const CTCODE: &'static str = "86";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path: &Path = Path::new("config.yml");
    let cookie_path: &Path = Path::new("cookie.json");
    let content = match fs::read_to_string(config_path).await {
        Ok(v) => v,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let _ = fs::write(config_path, Config::generate_default()).await;
                bail!("未读取到配置文件！已生成默认配置文件，请进行配置后重新运行本程序");
            }
            _ => {
                bail!(
                    "读取配置文件错误！可以尝试删除现有的配置文件并重新运行程序以自动生成默认配置文件",
                );
            }
        },
    };
    let config = Arc::new(match Config::load(&content.as_str()) {
        Ok(v) => v,
        Err(e) => match e {
            ConfigError::Parse(_) => {
                bail!("max_bitrate_level设置有误");
            }
            _ => {
                bail!("配置文件解析错误");
            }
        },
    });
    let api = Arc::new(Mutex::new(
        match File::open(cookie_path).map(BufReader::new) {
            Ok(reader) => {
                let Ok(cookie_jar) = cookie_store::serde::json::load(reader) else {
                    bail!("cookie文件解析错误，可以删除cookie.json重试");
                };
                MusicApi::from_cookie_jar(cookie_jar, MAX_CONS)
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    let result = MusicApi::new(MAX_CONS);
                    let _ = cli::print("请输入手机号码接收验证码以登录：").await;
                    let phone = cli::input().await.unwrap();
                    let Ok(_) = result.captcha(CTCODE.to_string(), phone.clone()).await else {
                        bail!("验证码发送失败！");
                    };
                    let _ = cli::print("请输入验证码：").await;
                    let captcha = cli::input().await.unwrap();
                    match result
                        .login_cellphone(CTCODE.to_string(), phone, captcha)
                        .await
                    {
                        Ok(info) => match info.code {
                            200 => {
                                let _ = cli::print("登录成功！").await;
                            }
                            _ => {
                                bail!("登录失败：{}", info.msg);
                            }
                        },
                        Err(_) => {
                            bail!("登录失败！");
                        }
                    };
                    result
                }
                _ => {
                    bail!("cookie文件读取错误，可以删除cookie.json重试");
                }
            },
        },
    ));
    {
        let Ok(mut writer) = std::fs::File::create(cookie_path).map(std::io::BufWriter::new) else {
            bail!("cookie文件写入失败，可以删除cookie.json重试");
        };
        let cookie_jar = api.lock().await.cookie_jar();
        let store = cookie_jar.lock().unwrap();
        let Ok(_) = cookie_store::serde::json::save(&store, &mut writer) else {
            bail!("cookie文件写入失败，可以删除cookie.json重试");
        };
    }
    match api.lock().await.login_status().await {
        Ok(info) => {
            let _ = cli::print(&format!("已以 {} 身份成功登录！", info.nickname)).await;
        }
        Err(e) => {
            bail!("登录错误：{}", e);
        }
    }
    let _ = cli::print("请输入要下载的歌单Id：").await;
    let Ok(playlist_id) = cli::input().await.unwrap().parse::<u64>() else {
        bail!("歌单Id格式错误！");
    };
    let Ok(playlist_detail) = api.lock().await.song_list_detail(playlist_id).await else {
        bail!("歌单Id错误！");
    };
    let folder_name = util::truncate_filename(&playlist_detail.name, MAX_NAME_LENGTH);
    let folder_path = Arc::new(PathBuf::from(folder_name));

    if Path::new(folder_path.as_ref()).exists() {
        let _ = fs::remove_dir_all(folder_path.as_ref()).await;
    }
    let _ = fs::create_dir(folder_path.as_ref()).await;
    let _ = cli::print(&format!("正在下载 {} 歌单", playlist_detail.name)).await;

    let progress_bar = ProgressBar::new(playlist_detail.songs.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("正在下载 [{bar}] {pos}/{len}")
            .unwrap()
            .progress_chars("=> "),
    );
    progress_bar.inc(0);
    let failed_songs: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let failed_lyrics: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let semaphore = Arc::new(Semaphore::new(config.concurrency));
    let mut join_handles = Vec::new();
    for song_info in playlist_detail.songs {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let failed_songs = failed_songs.clone();
        let failed_lyrics = failed_lyrics.clone();
        let api = api.clone();
        let config = config.clone();
        let folder_path = folder_path.clone();
        let progress_bar = progress_bar.clone();
        join_handles.push(tokio::spawn(async move {
            let song_file_base_name = format!(
                "{}{} - {}",
                song_info.name,
                match song_info.translated_name {
                    Some(v) => format!("({})", v),
                    None => format!(""),
                },
                song_info.singer.join(", ")
            );
            if config.download_songs {
                let Ok(song_url) = api
                    .lock()
                    .await
                    .songs_url(&[song_info.id], &config.max_bitrate_level)
                    .await
                else {
                    failed_songs.lock().await.push(song_file_base_name.clone());
                    return;
                };
                let song_url = song_url.first().unwrap();
                let song_file_name = format!("{}.{}", song_file_base_name, song_url.extension);
                let cover_file_name = format!("{}.jpg", song_file_base_name);
                let song_path = folder_path.join(song_file_name);
                let cover_path = folder_path.join(cover_file_name);
                let Ok(_) = download_file(
                    &url::Url::parse(&song_url.url).unwrap(),
                    &song_path,
                    DownloadOptions::new(config.retry, config.retry_delay, config.timeout),
                )
                .await
                else {
                    failed_songs.lock().await.push(song_file_base_name.clone());
                    return;
                };
                let ext = &song_url.extension;
                if ext == "mp3" || ext == "flac" {
                    let Ok(_) = download_file(
                        &url::Url::parse(&song_info.pic_url).unwrap(),
                        &cover_path,
                        DownloadOptions::new(config.retry, config.retry_delay, config.timeout),
                    )
                    .await
                    else {
                        failed_songs.lock().await.push(song_file_base_name.clone());
                        return;
                    };
                    let cover_data = match fs::read(&cover_path).await {
                        Ok(data) => data,
                        Err(_) => {
                            failed_songs.lock().await.push(song_file_base_name.clone());
                            return;
                        }
                    };
                    let track_info = TrackInfo {
                        title: &song_info.name,
                        artists: &song_info.singer.iter().map(|v| v.as_str()).collect(),
                        album: &song_info.album,
                        cover_data: &cover_data,
                        cover_mime_type: lofty::picture::MimeType::Jpeg,
                    };

                    if let Err(e) = write_metadata(ext, &song_path, &track_info) {
                        log::warn!(
                            "Failed to write metadata for {}: {}",
                            song_file_base_name,
                            e
                        );
                    }
                    if let Err(e) = fs::remove_file(&cover_path).await {
                        log::warn!("Failed to delete cover file: {}", e);
                    }
                }
            }
            if config.download_lyrics {
                let lyric_file_name = format!("{}.lrc", song_file_base_name);
                let lyric_path = folder_path.join(lyric_file_name);
                match api.lock().await.song_lyric(song_info.id).await {
                    Ok(v) => {
                        let lyric_content = v.lyric.join("\n");
                        let Ok(mut writer) = tokio::fs::File::create(lyric_path)
                            .await
                            .map(tokio::io::BufWriter::new)
                        else {
                            failed_lyrics.lock().await.push(song_file_base_name);
                            return;
                        };
                        let Ok(_) = writer.write_all(lyric_content.as_bytes()).await else {
                            failed_lyrics.lock().await.push(song_file_base_name);
                            return;
                        };
                        let Ok(_) = writer.flush().await else {
                            failed_lyrics.lock().await.push(song_file_base_name);
                            return;
                        };
                    }
                    Err(_) => {}
                };
            }
            progress_bar.inc(1);
            drop(permit);
        }));
    }
    for handle in join_handles {
        handle.await.unwrap();
    }
    let _ = cli::print("下载完成！").await;
    {
        let failed = failed_songs.lock().await;
        if failed.len() > 0 {
            let _ = cli::print(&format!("歌曲下载失败：{}", failed.join(", ")));
        }
    }
    {
        let failed = failed_lyrics.lock().await;
        if failed.len() > 0 {
            let _ = cli::print(&format!("歌词下载失败：{}", failed.join(", ")));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{}", util::truncate_filename("", MAX_NAME_LENGTH));
    }
}
