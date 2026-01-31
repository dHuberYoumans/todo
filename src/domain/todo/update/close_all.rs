use anyhow::Result;

use crate::domain::{Prio, TodoItemUpdate, TodoList};

impl TodoList {
    pub fn close_all(&self, repo: &impl TodoItemUpdate, prio: Option<Prio>) -> Result<()> {
        repo.close_all(prio)?;
        Ok(())
    }
}
