use anyhow::{Context, Result};

use crate::domain::{TodoList, TodoListCreate};

impl TodoList {
    pub fn add_list(&self, repo: &impl TodoListCreate, list: &str) -> Result<()> {
        repo.add(list).context("✘ Couldn't add new list")
    }
}
