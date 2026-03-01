use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub db_path: String,
}

/// Model for tracks in the db
#[derive(Debug, Clone)]
pub struct Track {
    pub id: u32,
    pub path: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub bitrate: u32,
    pub tagver: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Library {
    pub id: u32,
    pub name: String,
    pub path: String,
}
