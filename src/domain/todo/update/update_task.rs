use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn update_task(&self, repo: &impl TodoItemRepository, msg: &str, id: &str) -> Result<()> {
        repo.update_task(msg, id)?;
        Ok(())
    }
}
