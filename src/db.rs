use crate::models::{Library, Track};
use anyhow::Result;
use log::warn;
use rusqlite::Connection;
use std::path::PathBuf;

/// Connect to an in-memory SQLite database.
pub fn connect() -> Result<Connection> {
    let conn = Connection::open_in_memory()?;
    Ok(conn)
}

/// Create a library table in the database.
pub fn create_library_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE library (
            id INTEGER PRIMARY KEY,
            name STRING NOT NULL,
            path STRING NOT NULL
        )",
        (),
    )?;
    Ok(())
}

/// Create a track table in the database.
pub fn create_track_table(conn: &Connection) -> Result<()> {
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

/// For a given track struct, extract its values and insert into the db
pub fn insert_track(conn: &Connection, track: &Track) -> Result<()> {
    conn.execute(
        "INSERT INTO track (path, title, artist, album, bitrate, tagver) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            &track.path,
            &track.title,
            &track.artist,
            &track.album,
            &track.bitrate,
            &track.tagver,
        ),
    )?;
    Ok(())
}

/// For a given library struct, extract its values and insert into the db
pub fn insert_library(conn: &Connection, library: &Library) -> Result<()> {
    conn.execute(
        "INSERT INTO library (name, path) VALUES (?1, ?2)",
        (&library.name, &library.path),
    )?;
    Ok(())
}

pub fn index_paths(conn: &Connection, paths: &Vec<PathBuf>) -> Result<()> {
    for path in paths {
        let t = Track::from(path);
        if t.title.is_none() {
            warn!("Track {} has no title", path.display());
        }
        insert_track(conn, &t)?;
    }

    Ok(())
}
