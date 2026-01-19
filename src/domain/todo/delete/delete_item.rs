use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn delete_item(&mut self, repo: &impl TodoItemRepository, id: &str) -> Result<()> {
        repo.delete_item(id)?;
        Ok(())
    }
}
