use crate::paths::UserPaths;
use anyhow::Result;

pub fn clean_data() -> Result<()> {
    let user_paths = UserPaths::new();
    let db_file = user_paths.get_db()?;
    let db_parent = db_file.parent().unwrap_or(db_file.as_path());
    let config_file = user_paths.get_todo_config()?;
    let config_parent = config_file.parent().unwrap_or(config_file.as_path());
    log::debug!("found database under {}", db_parent.to_string_lossy());
    log::debug!(
        "found config file under {}",
        config_parent.to_string_lossy()
    );
    std::fs::remove_dir_all(db_parent)?;
    std::fs::remove_dir_all(config_parent)?;
    Ok(())
}
