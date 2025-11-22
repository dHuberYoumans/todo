use anyhow::{anyhow, Result};

use crate::domain::TodoList;
use crate::paths::UserPaths;

impl TodoList {
    pub fn show_paths(&self) -> Result<()> {
        let user_paths = UserPaths::new();
        if let Some(ref db_path) = self.db_path {
            println!("{}", db_path.parent().unwrap_or(db_path).to_string_lossy())
        } else {
            return Err(anyhow!("✘ No path to database found."));
        }
        if let Some(ref config_path) = user_paths.config {
            println!(
                "{}",
                config_path
                    .parent()
                    .unwrap_or(config_path)
                    .to_string_lossy()
            );
        } else {
            return Err(anyhow!("✘ No path to configuration file found."));
        }
        Ok(())
    }
}
