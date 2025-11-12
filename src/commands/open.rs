use anyhow::Result;

use crate::domain::Status;
use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn open(&self, repo: &impl TodoItemRepository, id: &str) -> Result<()> {
        repo.update_status(Status::Open, id)?;
        Ok(())
    }
}
