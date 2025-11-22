use anyhow::{anyhow, Result};
use std::fs;

use crate::domain::TodoList;
use crate::paths::UserPaths;
use crate::util;

impl TodoList {
    pub fn config(self) -> Result<()> {
        let user_paths = UserPaths::new();
        if let Some(ref path) = user_paths.config {
            log::info!("read config at {}", path.to_string_lossy());
            let config = fs::read_to_string(path).ok();
            let new_config = util::edit_in_editor(config);
            log::info!("write new config");
            fs::write(path, new_config)?;
            println!("✔ Config written");
            Ok(())
        } else {
            Err(anyhow!("✘ No config file found."))
        }
    }
}
