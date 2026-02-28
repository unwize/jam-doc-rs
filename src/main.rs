use anyhow::Result;
use std::env;
use rusqlite::{Connection};
use serde::Deserialize;

/// Representation of the structure of the config file
#[derive(Deserialize)]
struct Config {
    db_path: String
}

/// Model for tracks in the db
#[derive(Debug, Clone)]
struct Track {
    id: u32,
    path: String,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    bitrate: u32,
    tagver: Option<String>
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let conn = Connection::open_in_memory()?;

    // Set up track table in DB
    conn.execute(
        "CREATE TABLE track (
            id INTEGER PRIMARY KEY,
            path STRING NOT NULL,
            title STRING,
            artist STRING,
            album STRING,
            bitrate INTEGER NOT NULL,
            tagver STRING
        )",
        (),
    )?;
    Ok(())
}
