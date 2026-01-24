use anyhow::{anyhow, Context, Result};
use std::fs;
use std::io::Write;

use crate::adapters::cli::config::entities::Config;
use crate::paths::UserPaths;

pub fn init() -> Result<()> {
    let user_paths = UserPaths::new();
    let home = user_paths.home;
    log::info!("Creating default config file");
    if let Some(config) = user_paths.todo_config {
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
id_length = 6
due_date_display_format = "%x" # chrono strftime-style
due_date_input_format = "DMY" # MDY | ISO
show_due = true
show_tag = true
sort_by = "prio"  # prio | due | tag
table = "modern_rounded" # ascii | ascii_rounded | modern |  modern_rounded | markdown"#,
            db_path.to_string_lossy()
        )
        .context("✘ Couldn't write default config to file")?;
    } else {
        log::error!("Could not resolve XDG directories");
    }
    Ok(())
}

pub fn read() -> Result<Config> {
    let user_paths = UserPaths::new();
    if let Some(config_path) = user_paths.todo_config {
        let config_file = fs::read_to_string(&config_path).context("✘ Couldn't read file")?;
        log::debug!("reading config at {config_path:?}");
        let config: Config = toml::from_str(&config_file).context("✘ Couldn't parse toml")?;
        Ok(config)
    } else {
        Err(anyhow!("✘ Path to config not found"))
    }
}
