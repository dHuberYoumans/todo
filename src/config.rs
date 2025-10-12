use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::Write;
use toml;

use crate::paths::UserPaths;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub style: Style,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub todo_db: String,
}

#[derive(Debug, Deserialize)]
pub struct Style {
    pub color_by: String,
    pub sort_by: String,
}

impl Config {
    pub fn init() -> Result<(), Box<dyn Error>> {
        let user_paths = UserPaths::new();
        let home = user_paths.home;
        log::info!("Creating default config file");
        if let Some(config) = user_paths.config {
            if let Some(parent) = config.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut config_file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&config)?;
            let mut db_path = home.to_path_buf();
            db_path.push(".todo/todo.db");
            writeln!(
                config_file,
                r#"
[database]
todo_db = "{}"

[style]
color_by = "prio" #prio | due
sort_by = "prio"  #prio | due | tag"#,
                db_path.to_string_lossy()
            )?;
        } else {
            log::error!("CONFIG not found");
        }
        Ok(())
    }

    pub fn read() -> Result<Config, Box<dyn Error>> {
        let user_paths = UserPaths::new();
        if let Some(config_path) = user_paths.config {
            let config_file = fs::read_to_string(&config_path)?;
            log::debug!("reading config at {config_path:?}");
            let config: Config = toml::from_str(&config_file)?;
            Ok(config)
        } else {
            Err("✘ Path to config not found".into())
        }
    }
}
