use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

use crate::infrastructure;
use crate::infrastructure::UserPaths;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub style: Style,
    pub aliases: Option<HashMap<String, String>>,
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
    pub id_length: usize,
    pub due_date_display_format: String,
    pub due_date_input_format: String,
    pub show_due: bool,
    pub show_tag: bool,
    pub sort_by: String,
    pub table: String,
}

pub fn load_config() -> Result<Config> {
    let paths = UserPaths::new();
    let config = infrastructure::config::read_config(&paths)?;

    Ok(Config {
        database: config.database,
        style: config.style,
        aliases: config.aliases,
    })
}
