use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::{Datelike, Duration, Local, NaiveDate, Weekday};
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

use crate::domain::Datetime;
use crate::paths::UserPaths;

const TMP_FILE: &str = "./EDIT_TASK";

pub fn parse_date(input: &str) -> Result<Datetime> {
    let target = match input.to_lowercase().as_str() {
        "mon" => Some(Weekday::Mon),
        "tue" => Some(Weekday::Tue),
        "wed" => Some(Weekday::Wed),
        "thu" => Some(Weekday::Thu),
        "fri" => Some(Weekday::Fri),
        "sat" => Some(Weekday::Sat),
        "sun" => Some(Weekday::Sun),
        _ => None,
    };
    if let Some(target) = target {
        let today = Local::now();
        let mut date = today.date_naive();
        while date.weekday() != target {
            date += Duration::days(1);
        }
        let naive_dt = date.and_hms_opt(0, 0, 0).unwrap();
        let local_dt = Local.from_local_datetime(&naive_dt).unwrap();
        Ok(Datetime {
            timestamp: local_dt,
        })
    } else {
        match input {
            "today" => {
                let today = Local::now().date_naive();
                let naive_dt = today.and_hms_opt(0, 0, 0).unwrap();
                let local_dt = Local.from_local_datetime(&naive_dt).unwrap();
                Ok(Datetime {
                    timestamp: local_dt,
                })
            }
            "tomorrow" => {
                let today = Local::now().date_naive();
                let tomorrow = today.succ_opt().unwrap(); // safe until end of time
                let tomorrow_dt = Local
                    .from_local_datetime(&tomorrow.and_time(NaiveTime::MIN))
                    .unwrap();
                Ok(Datetime {
                    timestamp: tomorrow_dt,
                })
            }
            "yesterday" => {
                let today = Local::now().date_naive();
                let yesteday = today.pred_opt().unwrap(); // safe until end of time
                let yesterday_dt = Local
                    .from_local_datetime(&yesteday.and_time(NaiveTime::MIN))
                    .unwrap();
                Ok(Datetime {
                    timestamp: yesterday_dt,
                })
            }
            _ => {
                let date = NaiveDate::parse_from_str(input, "%d-%m-%Y")
            .map_err(|_| anyhow!("✘ Invalid date format.\nUse either of the following:\n* today\n* tomorrow\n* 3 letter days for the next weekday\n* dd-mm-yyyy for a specific day"))?;
                let naive_dt = date.and_hms_opt(0, 0, 0).unwrap();
                let local_dt = Local.from_local_datetime(&naive_dt).single().unwrap();
                Ok(Datetime {
                    timestamp: local_dt,
                })
            }
        }
    }
}

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
