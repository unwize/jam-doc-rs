use std::path::PathBuf;

use id3::{Tag, TagLike};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub db_path: String,
}

/// Model for tracks in the db
#[derive(Debug, Clone)]
pub struct Track {
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub bitrate: u32,
    pub tagver: Option<String>,
}

impl From<&PathBuf> for Track {
    fn from(path: &PathBuf) -> Self {
        let p = path.to_string_lossy();
        let metadata = Tag::read_from_path(path).unwrap_or_default();

        Track {
            path: p.into_owned(),
            title: metadata.title().map(|t| t.to_string()),
            artist: metadata.artist().map(|a| a.to_string()),
            album: metadata.album().map(|a| a.to_string()),
            bitrate: 0,
            tagver: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Library {
    pub id: u32,
    pub name: String,
    pub path: String,
}
