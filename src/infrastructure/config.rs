use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::application::config::Config;
use crate::infrastructure::paths::UserPaths;

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
