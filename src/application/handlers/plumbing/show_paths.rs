use crate::infrastructure::config::{get_todo_config, read_config};
use crate::infrastructure::paths::UserPaths;
use anyhow::{anyhow, Result};

pub fn show_paths() -> Result<()> {
    let user_paths = UserPaths::new();
    let config = read_config(&user_paths)?;
    let db_path = config.database.todo_db;
    let config = get_todo_config(&user_paths)?;
    println!("{:<16} {}", "home:", user_paths.home.display());
    println!(
        "{:<16} {}",
        "config:",
        user_paths
            .config
            .clone()
            .ok_or(anyhow!(
                "✘ No standard location for configuration files found"
            ))?
            .to_string_lossy()
    );
    println!("{:<16} {}", "todo.config at:", config.to_string_lossy());
    println!("{:<16} {}", "database at:", db_path);
    Ok(())
}
