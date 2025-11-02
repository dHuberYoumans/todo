use anyhow::Result;

use crate::domain::{TodoList, TodoListRepository};
use crate::persistence::{SqlTodoItemRepository, SqlTodoListRepository};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::util;
use crate::{config::Config, paths::UserPaths};

const INIT_LIST: &'static str ="todo";

impl TodoList {
    pub fn init(
        &mut self,
    ) -> Result<()> {
        println!("⧖ Initializing..");
        let user_paths = UserPaths::new();
        let env_path = prepare_environment_path(&user_paths)?;
        println!("⧖ Setting up database..");
        set_up_environment(&env_path, &user_paths)?;
        Config::init()?;
        self.db_path = util::get_db_path();
        log::info!("creating database at {}", util::log_opt_path(&self.db_path));
        let conn = util::connect_to_db(&self.db_path)?;
        let todo_list_repo = SqlTodoListRepository::new(&conn);
        let todo_item_repo = SqlTodoItemRepository::new(&conn, INIT_LIST.to_string());
        log::info!("creating new collection");
        todo_list_repo.create_table()?;
        log::info!("creating new table");
        self.new_list(&todo_list_repo, &todo_item_repo, String::from("todo"), false)?;
        println!("✔ All done");
        Ok(())
    }
}


fn prepare_environment_path(user_paths: &UserPaths) -> Result<PathBuf> {
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
            return Ok(env_path);
        }
    Ok(env_path)
}

fn set_up_environment(env_path: &PathBuf, user_paths: &UserPaths) -> Result<()> {
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
    writeln!(env, "CURRENT={}", INIT_LIST)?;
    writeln!(env, "PREVIOUS={}", INIT_LIST)?;
    Ok(())
}
