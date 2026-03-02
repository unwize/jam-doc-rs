use anyhow::Result;
use std::env;
use std::path::PathBuf;

use crate::db::{create_library_table, create_track_table, index_paths};

mod db;
mod models;
mod scan;
/// Representation of the structure of the config file

#[tokio::main]
async fn main() -> Result<()> {
    colog::init();
    let args: Vec<String> = env::args().collect();

    let conn = db::connect()?;

    create_library_table(&conn)?;
    create_track_table(&conn)?;

    let paths = scan::scan_dir(PathBuf::from(r"V:\Music\Music"))?;
    index_paths(&conn, &paths)?;

    Ok(())
}
