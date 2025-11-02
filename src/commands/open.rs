use anyhow::Result;

use crate::domain::Status;
use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn open(&mut self, repo: &impl TodoItemRepository, id: i64) -> Result<()> {
        let current = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current);
        repo.update_status(Status::Open, id)?;
        Ok(())
    }
}
