use anyhow::{anyhow, Result};
use dirs::home_dir;
use glob;
use log;
use rusqlite::Connection;
use std::{
    env, fs,
    io::Read,
    path::{Path, PathBuf},
    process,
};

use crate::paths::UserPaths;

const TMP_FILE: &str = "./EDIT_TASK";

pub fn get_todo_dir() -> Option<PathBuf> {
    Some(home_dir()?.join(".todo"))
}

pub fn edit_in_editor(old_text: Option<String>) -> String {
    let editor = env::var("EDITOR").unwrap_or(String::from("vi"));
    let path = PathBuf::from(TMP_FILE);
    fs::File::create(&path).unwrap_or_else(|_| panic!("✘ Could not open file {}", TMP_FILE));
    if let Some(text) = old_text {
        fs::write(&path, text).unwrap_or_else(|_| panic!("✘ Could not write to file {}", TMP_FILE));
    };
    process::Command::new(editor)
        .arg(&path)
        .status()
        .expect("✘ Couldn't open your editor");
    let mut task = String::new();
    fs::File::open(&path)
        .unwrap_or_else(|_| panic!("✘ Could not open file {}", TMP_FILE))
        .read_to_string(&mut task)
        .expect("✘ Couldn't parse task");
    cleanup_tmp_files().expect("✘ Error during cleanup");
    task
}

fn cleanup_tmp_files() -> Result<()> {
    let pattern = format!("{TMP_FILE}*");
    for file in glob::glob(&pattern)? {
        match file {
            Ok(path) => std::fs::remove_file(path)?,
            Err(e) => eprintln!("✘ Could not find file: {e}"),
        }
    }
    Ok(())
}

pub fn connect_to_db(db: &PathBuf) -> Result<Connection> {
    log::info!("connecting to database at {}", log_opt_path(db));
    let conn = Connection::open(db)?;
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    Ok(conn)
}

pub fn load_env() -> Result<()> {
    let env_path = UserPaths::new().home.join(".todo").join(".env");
    if dotenv::from_filename(&env_path).is_err() {
        return Err(anyhow!("✘ No .env file found at {env_path:?}"));
    }
    Ok(())
}

pub fn dotenv() -> Result<PathBuf> {
    if let Some(home) = home_dir() {
        Ok(home.join("./todo/.env"))
    } else {
        Err(anyhow!(
            "✘ No path to database found. Consider 'todo init' to initialize a data base"
        ))
    }
}

pub fn log_opt_path(p: &Path) -> String {
    p.to_string_lossy().into()
}
