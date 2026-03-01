use crate::models::{Library, Track};
use anyhow::Result;
use rusqlite::Connection;

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
            library_id INTEGER NOT NULL,
            title STRING NOT NULL,
            artist STRING,
            album STRING,
            bitrate INTEGER NOT NULL,
            tagver STRING,
            FOREIGN KEY (library_id) REFERENCES library(id)
        )",
        (),
    )?;
    Ok(())
}

/// For a given track struct, extract its values and insert into the db
pub fn insert_track(conn: &Connection, track: &Track) -> Result<()> {
    conn.execute(
        "INSERT INTO track (library_id, title, artist, album, bitrate, tagver) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (track.id, &track.title, &track.artist, &track.album, &track.bitrate, &track.tagver),
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
