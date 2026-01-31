use anyhow::Result;

use crate::domain::{TodoItemDelete, TodoList};

impl TodoList {
    pub fn delete_item(&mut self, repo: &impl TodoItemDelete, id: &str) -> Result<()> {
        repo.delete_item(id)?;
        Ok(())
    }
}
