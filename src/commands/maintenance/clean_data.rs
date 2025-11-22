use crate::domain::TodoList;
use crate::paths::UserPaths;
use anyhow::{anyhow, Result};

impl TodoList {
    pub fn clean_data(&self) -> Result<()> {
        let user_paths = UserPaths::new();
        let db_path = if let Some(ref db_path) = self.db_path {
            db_path.parent().unwrap_or(db_path)
        } else {
            return Err(anyhow!("✘ No path to database found. Nothing to clean."));
        };
        let config_path = if let Some(ref config_path) = user_paths.config {
            config_path.parent().unwrap_or(config_path)
        } else {
            return Err(anyhow!(
                "✘ No path to configuration file found. Nothing to clean."
            ));
        };
        std::fs::remove_dir_all(db_path)?;
        std::fs::remove_dir_all(config_path)?;
        Ok(())
    }
}
