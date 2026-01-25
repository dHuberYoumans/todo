use anyhow::{Context, Result};
use log;
use rusqlite::Connection;
use std::path::PathBuf;

pub fn connect_to_db(db: &PathBuf) -> Result<Connection> {
    log::info!("connecting to database at {}", db.display());
    let conn = Connection::open(db).context("✘ Couldn't connect to database")?;
    conn.execute("PRAGMA foreign_keys = ON;", [])
        .context("✘ Couldn't set option 'foreign_keys' in database")?;
    Ok(conn)
}
