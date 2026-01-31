use anyhow::Result;

use crate::domain::{TodoItemDelete, TodoList};

impl TodoList {
    pub fn delete_all_items(&self, repo: &impl TodoItemDelete) -> Result<()> {
        repo.delete_all_items()?;
        Ok(())
    }
}
