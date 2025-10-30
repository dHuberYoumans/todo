use std::error::Error;

use crate::todo::TodoList;
use rusqlite::{Connection, Result};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::queries::collection::Collection;
use crate::util;
use crate::{config::Config, paths::UserPaths};

impl TodoList {
    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        println!("⧖ Initializing..");
        let user_paths = UserPaths::new();
        let home = if let Ok(path) = std::env::var("HOME") {
            // hijack env for testing
            PathBuf::from(path)
        } else {
            user_paths.home.clone()
        };
        log::debug!("$HOME={:?}", &home);
        let mut env_path = home.to_path_buf();
        env_path.push(".todo/.env");
        if env_path.exists() {
            println!("✔ Environmental setup found");
            return Ok(());
        }
        println!("⧖ Setting up database..");
        fs::create_dir_all(env_path.parent().unwrap())?;
        let mut env = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(env_path)?;
        log::debug!("$CONFIG={:?}", user_paths.config);
        if let Some(config) = user_paths.config.clone() {
            writeln!(env, "CONFIG={}", config.to_string_lossy())?;
        } else {
            log::debug!("CONFIG not found. Setting CONFIG=");
            writeln!(env, "CONFIG=")?;
        }
        Config::init()?;
        writeln!(env, "CURRENT=todo")?;
        writeln!(env, "PREVIOUS=todo")?;
        self.db_path = util::get_db_path();
        log::info!("creating database at {}", util::log_opt_path(&self.db_path));
        let conn = if let Some(path) = &self.db_path {
            Connection::open(path)?
        } else {
            return Err("✘ Something went wrong setting up the the database"
                .to_string()
                .into());
        };
        log::info!("creating new collection");
        Collection::create_table(&conn)?;
        // conn.execute(&queries::create_collection(), [])?;
        log::info!("creating new table");
        self.new_list(String::from("todo"), false)?;
        println!("✔ All done");
        Ok(())
    }
}
