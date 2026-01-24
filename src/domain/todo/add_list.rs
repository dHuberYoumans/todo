use anyhow::Result;

use crate::domain::{TodoList, TodoListRepository};

impl TodoList {
    pub fn add_list(&self, repo: &impl TodoListRepository, list: &str) -> Result<()> {
        repo.add(list)
    }
}
