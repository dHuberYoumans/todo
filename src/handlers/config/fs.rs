use anyhow::{Context, Result};
use std::fs;
use std::io::Write;

use crate::infrastructure::paths::UserPaths;

pub fn init() -> Result<()> {
    let user_paths = UserPaths::new();
    let home = user_paths.home;
    log::info!("Creating default config file");
    if let Some(config) = user_paths.todo_config {
        if let Some(parent) = config.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config)?;
        let mut db_path = home.to_path_buf();
        db_path.push(".todo/todo.db");
        writeln!(
            config_file,
            r#"
[database]
todo_db = "{}"

[style]
id_length = 6
due_date_display_format = "%x" # chrono strftime-style
due_date_input_format = "DMY" # MDY | ISO
show_due = true
show_tag = true
sort_by = "prio"  # prio | due | tag
table = "modern_rounded" # ascii | ascii_rounded | modern |  modern_rounded | markdown"#,
            db_path.to_string_lossy()
        )
        .context("✘ Couldn't write default config to file")?;
    } else {
        log::error!("Could not resolve XDG directories");
    }
    Ok(())
}
