use anyhow::Result;

use std::env;

use crate::db::{create_library_table, create_track_table};

mod db;
mod models;
mod scan;
/// Representation of the structure of the config file

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let conn = db::connect()?;

    create_library_table(&conn)?;
    create_track_table(&conn)?;

    Ok(())
}
