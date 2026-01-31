use anyhow::{Context, Result};

use crate::domain::{TodoList, TodoListRead};

impl TodoList {
    pub fn get_collection(&self, repo: &impl TodoListRead) -> Result<Vec<String>> {
        repo.fetch_all().context("✘ Couldn't fetch collection")
    }
}
