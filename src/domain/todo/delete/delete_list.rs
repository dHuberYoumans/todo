use anyhow::Result;

use crate::domain::{TodoList, TodoListDelete};

impl TodoList {
    pub fn delete_list(&self, repo: &impl TodoListDelete, list: &str) -> Result<()> {
        repo.delete(list)?;
        Ok(())
    }
}
