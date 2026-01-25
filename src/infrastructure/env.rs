use anyhow::{anyhow, Result};
use std::path::PathBuf;

use crate::infrastructure::UserPaths;

pub fn load_env(user_paths: &UserPaths) -> Result<()> {
    let env_path = dotenv(user_paths);
    if dotenv::from_filename(&env_path).is_err() {
        return Err(anyhow!("✘ No .env file found at {env_path:?}"));
    }
    Ok(())
}

pub fn dotenv(user_paths: &UserPaths) -> PathBuf {
    user_paths.home.join(".todo/.env")
}
