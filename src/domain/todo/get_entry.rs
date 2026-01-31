use anyhow::Result;

use crate::domain::{TodoItemQuery, TodoList};

impl TodoList {
    pub fn get_entry(&mut self, repo: &impl TodoItemQuery, id: &str) -> Result<Option<String>> {
        let entry = repo.fetch_task_by_id(id)?;
        Ok(entry)
    }
}
