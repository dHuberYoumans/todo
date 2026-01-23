use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn delete_all_items(&self, repo: &impl TodoItemRepository) -> Result<()> {
        repo.delete_all_items()?;
        Ok(())
    }
}
