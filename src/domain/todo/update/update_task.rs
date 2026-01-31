use anyhow::Result;

use crate::domain::{TodoItemUpdate, TodoList};

impl TodoList {
    pub fn update_task(&self, repo: &impl TodoItemUpdate, msg: &str, id: &str) -> Result<()> {
        repo.update_task(msg, id)?;
        Ok(())
    }
}
