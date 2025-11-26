use anyhow::Result;

use crate::domain::Status;
use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn close(&self, repo: &impl TodoItemRepository, ids: Vec<String>) -> Result<()> {
        repo.update(None, None, Some(Status::Closed), None, ids)?;
        Ok(())
    }
}
