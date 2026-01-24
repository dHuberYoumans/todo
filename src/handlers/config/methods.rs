use anyhow::{anyhow, Result};
use std::fs;

use crate::paths::UserPaths;
use crate::util;

pub fn edit() -> Result<()> {
    let user_paths = UserPaths::new();
    if let Some(ref path) = user_paths.todo_config {
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
