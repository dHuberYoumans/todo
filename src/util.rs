use anyhow::{anyhow, Context, Result};
use dirs::home_dir;
use log;
use rusqlite::Connection;
use std::path::{Path, PathBuf};

use crate::infrastructure::paths::UserPaths;

pub fn get_todo_dir() -> Option<PathBuf> {
    Some(home_dir()?.join(".todo"))
}

pub fn connect_to_db(db: &PathBuf) -> Result<Connection> {
    log::info!("connecting to database at {}", log_opt_path(db));
    let conn = Connection::open(db).context("✘ Couldn't connect to database")?;
    conn.execute("PRAGMA foreign_keys = ON;", [])
        .context("✘ Couldn't set option 'foreign_keys' in database")?;
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
        Ok(home.join(".todo/.env"))
    } else {
        Err(anyhow!(
            "✘ No path to database found. Consider 'todo init' to initialize a data base"
        ))
    }
}

pub fn log_opt_path(p: &Path) -> String {
    p.to_string_lossy().into()
}

pub fn parse_task(task: &str) -> (String, String) {
    let (title, rest) = task.split_once('\n').unwrap_or((task, ""));
    let message = rest
        .lines()
        .skip_while(|line| line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    (title.to_string(), message)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_parse_task() {
        let task = "Title\nMessage";
        let expected_title = "Title".to_string();
        let expected_message = "Message".to_string();
        let parsed_task = parse_task(task);
        let (title, message) = parsed_task;
        assert_eq!(title, expected_title);
        assert_eq!(message, expected_message);

        let task = "Title";
        let expected_title = "Title".to_string();
        let expected_message = "".to_string();
        let parsed_task = parse_task(task);
        let (title, message) = parsed_task;
        assert_eq!(title, expected_title);
        assert_eq!(message, expected_message);

        let task = "Title\n\nParagraph1\nParagraph2";
        let expected_title = "Title".to_string();
        let expected_message = "Paragraph1\nParagraph2".to_string();
        let parsed_task = parse_task(task);
        let (title, message) = parsed_task;
        assert_eq!(title, expected_title);
        assert_eq!(message, expected_message);
    }
}
