use anyhow::Result;

use crate::domain::{TodoList, TodoListRepository};

impl TodoList {
    pub fn create_collection(&self, repo: &impl TodoListRepository) -> Result<()> {
        repo.create_table()
    }
}
