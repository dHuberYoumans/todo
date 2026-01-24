use anyhow::{anyhow, Context, Result};

use crate::domain::TodoList;
use crate::persistence::{SqlTodoItemRepository, SqlTodoListRepository};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::adapters::config;
use crate::paths::UserPaths;
use crate::util;

const INIT_LIST: &str = "todo";

pub fn init() -> Result<()> {
    println!("▶ Initializing...");
    let user_paths = UserPaths::new();
    let env_path = prepare_environment_path(&user_paths);
    println!("▶ Setting up database...");
    config::fs::init()?;
    set_up_environment(&env_path)?;
    let db_path = user_paths.get_db()?;
    log::info!("creating database at {}", util::log_opt_path(&db_path));
    let conn = util::connect_to_db(&db_path)?;
    let todo_list_repo = SqlTodoListRepository::new(&conn);
    let todo_item_repo = SqlTodoItemRepository::new(&conn, INIT_LIST.to_string());
    let todo_list = TodoList::new();
    log::info!("creating new collection");
    todo_list.create_collection(&todo_list_repo)?;
    todo_list.add_list(&todo_list_repo, INIT_LIST)?;
    log::info!("creating new table");
    todo_list.create_table(&todo_item_repo)?;
    println!("✔ All done");
    Ok(())
}

fn prepare_environment_path(user_paths: &UserPaths) -> PathBuf {
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
        return env_path;
    }
    env_path
}

fn set_up_environment(env_path: &PathBuf) -> Result<()> {
    let parent = env_path.parent().ok_or(anyhow!(
        "✘ No parent directory found for {}",
        env_path.display()
    ))?;
    fs::create_dir_all(parent).context("✘ Couldn't create directory")?;
    let mut env = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(env_path)?;
    writeln!(env, "CURRENT={}", INIT_LIST).context("✘ Couldn't write to .env")?;
    writeln!(env, "PREVIOUS={}", INIT_LIST).context("✘ Couldn't write to .env")?;
    Ok(())
}
