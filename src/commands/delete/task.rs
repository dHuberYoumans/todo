use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn delete(&mut self, repo: &impl TodoItemRepository, id: i64) -> Result<()> {
        repo.delete_task(id)?;
        Ok(())
    }
}
