use crate::adapters::cli::config;
use anyhow::{anyhow, Result};
use microxdg::Xdg;
use std::{path::PathBuf, str::FromStr};

#[derive(Clone)]
pub struct UserPaths {
    pub home: PathBuf,
    pub config: Option<PathBuf>,
    pub todo_config: Option<PathBuf>,
}

impl Default for UserPaths {
    fn default() -> Self {
        UserPaths::new()
    }
}

impl UserPaths {
    pub fn new() -> Self {
        let xdg = Xdg::new().expect("✘ Could not reslove XDG directories");
        let home = xdg.home().to_path_buf();
        let config = xdg.config().ok();
        let todo_config = xdg.config().map(|conf| conf.join("todo/todo.config")).ok();
        Self {
            home,
            config,
            todo_config,
        }
    }

    pub fn print_paths(&self) -> Result<()> {
        let db_path = self.get_db()?;
        let config = self.get_todo_config()?;
        println!("{:<16} {}", "home:", self.home.to_string_lossy());
        println!(
            "{:<16} {}",
            "config:",
            self.config
                .clone()
                .ok_or(anyhow!(
                    "✘ No standard location for configuration files found"
                ))?
                .to_string_lossy()
        );
        println!("{:<16} {}", "todo.config at:", config.to_string_lossy());
        println!("{:<16} {}", "database at:", db_path.to_string_lossy());
        Ok(())
    }

    pub fn get_db(&self) -> Result<PathBuf> {
        let config = config::fs::read()?;
        log::debug!("found config: {config:?}");
        let db_path = PathBuf::from_str(&config.database.todo_db)?;
        Ok(db_path)
    }

    pub fn get_config(&self) -> Result<PathBuf> {
        if let Some(ref path) = self.config {
            Ok(path.clone())
        } else {
            Err(anyhow!("✘ No default path for configuration files found"))
        }
    }

    pub fn get_todo_config(&self) -> Result<PathBuf> {
        if let Some(ref path) = self.todo_config {
            Ok(path.clone())
        } else {
            Err(anyhow!("✘ No configuration file found"))
        }
    }
}
