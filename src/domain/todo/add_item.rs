use anyhow::Result;

use crate::domain::{TodoItem, TodoItemCreate, TodoList};

impl TodoList {
    pub fn add_item(&self, repo: &impl TodoItemCreate, item: &TodoItem) -> Result<()> {
        repo.add(item)?;
        Ok(())
    }
}
