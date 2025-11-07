use anyhow::Result;

use crate::domain::Status;
use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn close(&self, repo: &impl TodoItemRepository, id: i64) -> Result<()> {
        repo.update_status(Status::Closed, id)?;
        Ok(())
    }
}
