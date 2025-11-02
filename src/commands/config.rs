use std::error::Error;
use std::fs;

use crate::domain::TodoList;
use crate::util;

impl TodoList {
    pub fn config(self) -> Result<(), Box<dyn Error>> {
        let path = std::env::var("CONFIG")?;
        log::info!("read config at {path}");
        let config = fs::read_to_string(&path).ok();
        let new_config = util::edit_in_editor(config);
        log::info!("write new config");
        fs::write(path, new_config)?;
        println!("✔ Config written");
        Ok(())
    }
}
