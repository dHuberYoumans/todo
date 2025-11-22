use crate::domain::TodoList;
use crate::paths::UserPaths;
use anyhow::Result;

impl TodoList {
    pub fn show_paths() -> Result<()> {
        let user_paths = UserPaths::new();
        user_paths.print_paths()?;
        Ok(())
    }
}
