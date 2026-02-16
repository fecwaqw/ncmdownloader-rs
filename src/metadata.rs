use anyhow::Result;
use lofty::{
    config::WriteOptions,
    file::{AudioFile, TaggedFileExt},
    picture::{MimeType, Picture, PictureType},
    probe::Probe,
    tag::{ItemKey, ItemValue, Tag, TagItem, TagType},
};
use std::path::Path;

pub struct TrackInfo<'a> {
    /// Title of the track
    pub title: &'a str,
    /// Artist(s) of the track
    pub artists: &'a Vec<&'a str>,
    /// Album name
    pub album: &'a str,
    /// Cover art image data (JPEG or PNG)
    pub cover_data: &'a [u8],
    /// MIME type of the cover image (typically "image/jpeg" or "image/png")
    pub cover_mime_type: MimeType,
}

pub fn write_metadata(extension: &str, path: &Path, info: &TrackInfo) -> Result<()> {
    let tag_type = match extension {
        "mp3" => TagType::Id3v2,
        "flac" => TagType::VorbisComments,
        _ => {
            return Ok(());
        }
    };
    let mut tagged_file = Probe::open(path)?.read()?;
    let tag = match tagged_file.primary_tag_mut() {
        Some(tag) => tag,
        None => {
            // 如果文件还没有标签，创建一个新的 ID3v2 标签
            &mut tagged_file.insert_tag(Tag::new(tag_type)).unwrap()
        }
    };
    tag.insert_text(ItemKey::TrackTitle, info.title.to_string());
    for artist in info.artists {
        tag.push(TagItem::new(
            ItemKey::TrackArtist,
            ItemValue::Text(artist.to_string()),
        ));
    }
    tag.insert_text(ItemKey::AlbumTitle, info.album.to_string());
    let picture = Picture::new_unchecked(
        PictureType::CoverFront,
        Some(info.cover_mime_type.clone()),
        None,
        info.cover_data.to_vec(),
    );
    tag.set_picture(0, picture);
    tagged_file.save_to_path(path, WriteOptions::default())?;
    Ok(())
}
