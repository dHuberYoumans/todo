use anyhow::Result;

use crate::domain::{TodoList, TodoListRepository};

impl TodoList {
    pub fn delete_list(&self, repo: &impl TodoListRepository, list: &str) -> Result<()> {
        repo.delete(list)?;
        Ok(())
    }
}
