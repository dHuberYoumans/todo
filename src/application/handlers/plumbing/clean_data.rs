use anyhow::Result;
use std::path::PathBuf;

pub fn clean_data(config_file: PathBuf, db_file: PathBuf) -> Result<()> {
    let db_parent = db_file.parent().unwrap_or(db_file.as_path());
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
