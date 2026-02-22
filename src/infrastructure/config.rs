use anyhow::{anyhow, Context, Result};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::application::config::Config;
use crate::application::editor::Editor;
use crate::infrastructure::paths::UserPaths;

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
            r#"[database]
todo_db = "{}"

[style]
id_length = 6
due_date_display_format = "%x" # chrono strftime-style
due_date_input_format = "DMY" # MDY | ISO
show_due = true
show_tag = true
sort_by = "prio"  # prio | due | tag
table = "modern_rounded" # ascii | ascii_rounded | modern |  modern_rounded | markdown

[aliases]
p1 = "add --prio=p1",
p2 = "add --prio=p2",
p3 = "add --prio=p3"#,
            db_path.to_string_lossy()
        )
        .context("✘ Couldn't write default config to file")?;
    } else {
        log::error!("Could not resolve XDG directories");
    }
    Ok(())
}

pub fn read_config(user_paths: &UserPaths) -> Result<Config> {
    if let Some(config_path) = user_paths.todo_config.clone() {
        let config_file = fs::read_to_string(&config_path).context("✘ Couldn't read file")?;
        log::debug!("reading config at {config_path:?}");
        let config: Config = toml::from_str(&config_file).context("✘ Couldn't parse toml")?;
        Ok(config)
    } else {
        Err(anyhow!("✘ Path to config not found"))
    }
}

pub fn edit_config(editor: &impl Editor) -> Result<()> {
    let user_paths = UserPaths::new();
    if let Some(ref path) = user_paths.todo_config {
        log::info!("read config at {}", path.to_string_lossy());
        let config = fs::read_to_string(path).ok();
        let new_config = editor.edit(config)?;
        log::info!("write new config");
        fs::write(path, new_config)?;
        println!("✔ Config written");
        Ok(())
    } else {
        Err(anyhow!("✘ No config file found."))
    }
}

pub fn get_config(user_paths: &UserPaths) -> Result<PathBuf> {
    if let Some(ref path) = user_paths.config {
        Ok(path.clone())
    } else {
        Err(anyhow!("✘ No default path for configuration files found"))
    }
}

pub fn get_todo_config(user_paths: &UserPaths) -> Result<PathBuf> {
    if let Some(ref path) = user_paths.todo_config {
        Ok(path.clone())
    } else {
        Err(anyhow!("✘ No configuration file found"))
    }
}
