use anyhow::Result;

use crate::domain::{TodoItem, TodoItemRepository, TodoList};

impl TodoList {
    pub fn add_item(&self, repo: &impl TodoItemRepository, item: &TodoItem) -> Result<()> {
        repo.add(item)?;
        Ok(())
    }
}
