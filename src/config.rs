use anyhow::{anyhow, Result};
use serde::Deserialize;
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
pub enum TableStyle {
    Ascii,
    AsciiRounded,
    Modern,
    ModernRounded,
    Markdown,
}

impl From<String> for TableStyle {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "ascii" => TableStyle::Ascii,
            "ascii_rounded" | "ascii-rounded" => TableStyle::AsciiRounded,
            "modern" => TableStyle::Modern,
            "modern_rounded" | "modern-rounded" => TableStyle::ModernRounded,
            "markdown" => TableStyle::Markdown,
            _ => TableStyle::ModernRounded,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Style {
    pub prefix_id_length: usize,
    pub sort_by: String,
    pub table: String,
}

impl Config {
    pub fn init() -> Result<()> {
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
prefix_id_length = 6
sort_by = "prio"  # prio | due | tag
table = "modern_rounded" # ascii | ascii_rounded | modern |  modern_rounded | markdown"#,
                db_path.to_string_lossy()
            )?;
        } else {
            log::error!("Could not resolve XDG directories");
        }
        Ok(())
    }

    pub fn read() -> Result<Config> {
        let user_paths = UserPaths::new();
        if let Some(config_path) = user_paths.config {
            let config_file = fs::read_to_string(&config_path)?;
            log::debug!("reading config at {config_path:?}");
            let config: Config = toml::from_str(&config_file)?;
            Ok(config)
        } else {
            Err(anyhow!("✘ Path to config not found"))
        }
    }
}
