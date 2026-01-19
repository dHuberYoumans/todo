use anyhow::Result;

use crate::domain::{Prio, TodoItemRepository, TodoList};

impl TodoList {
    pub fn close_all(&self, repo: &impl TodoItemRepository, prio: Option<Prio>) -> Result<()> {
        repo.close_all(prio)?;
        Ok(())
    }
}
