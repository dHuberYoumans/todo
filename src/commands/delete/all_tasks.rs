use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn delete_all(&mut self, repo: &impl TodoItemRepository) -> Result<()> {
        let ids = repo.fetch_all_ids()?;
        for id in ids.iter() {
            repo.delete_task(*id)?;
        }
        Ok(())
    }
}
