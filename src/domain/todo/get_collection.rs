use anyhow::Result;

use crate::domain::{TodoList, TodoListRepository};

impl TodoList {
    pub fn get_collection(&self, repo: &impl TodoListRepository) -> Result<Vec<String>> {
        repo.fetch_all()
    }
}
