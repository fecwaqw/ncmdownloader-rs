use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationMilliSeconds};
use std::time::Duration;
const BITRATE_LEVELS: [&str; 8] = [
    "higer", "exhigh", "lossless", "hires", "jyeffect", "sky", "dolby", "jymaster",
];

const DEFAULT_CONFIG: &str = r##"max_bitrate_level: "exhigh"
#max_bitrate_level:下载歌曲的最高质量，不填写内容默认为可下载的最高质量
#可填内容:
# higher => 较高
# exhigh=>极高
# lossless=>无损
# hires=>Hi-Res
# jyeffect => 高清环绕声
# sky => 沉浸环绕声
# dolby => 杜比全景声
# jymaster => 超清母带
download_songs: true
#download_songs:是否下载歌曲
#可填内容:
# true => 下载歌曲
# false => 不下载歌曲
download_lyrics: false
#download_lyrics:是否下载歌词
#可填内容:
# true => 下载歌词
# false => 不下载歌词
concurrency: 3
#concurrency:同时下载的任务数
#可填内容:正整数(不建议设置太大)
retry: 3
#retry:下载失败时的重试次数
#可填内容:正整数
retry_delay: 1000
#retry_delay:下载失败时的重试间隔时间
#可填内容:正整数(单位：毫秒)
timeout: 30000
#timeout:下载超时时间
#可填内容:正整数(单位：毫秒)
"##;
#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub max_bitrate_level: String,
    pub download_songs: bool,
    pub download_lyrics: bool,
    pub concurrency: usize,
    pub retry: usize,
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub retry_delay: Duration,
    #[serde_as(as = "DurationMilliSeconds<u64>")]
    pub timeout: Duration,
}

#[allow(unused)]
impl Config {
    pub fn load(content: &str) -> Result<Config, ConfigError> {
        let config: Config = serde_yaml::from_str(content).with_context(|| "配置解析错误")?;
        if !BITRATE_LEVELS
            .iter()
            .any(|&x| x == config.max_bitrate_level)
        {
            return Err(ConfigError::InvalidConfig(
                "max_bitrate_level设置有误".into(),
            ));
        }
        Ok(config)
    }

    pub fn generate_default() -> &'static str {
        DEFAULT_CONFIG
    }

    pub fn save(&self) -> Result<String, ConfigError> {
        Ok(serde_yaml::to_string(&self).with_context(|| "无法保存")?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("配置格式错误: {0}")]
    InvalidConfig(String),

    #[error("配置解析错误: {0}")]
    Parse(#[from] anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let config =
            Config::load(&std::fs::read_to_string(std::path::Path::new("config.yml")).unwrap())
                .unwrap();
        println!("{:?}", config.save());
    }
}
