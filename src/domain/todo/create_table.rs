use anyhow::{Context, Result};

use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn create_table(&self, repo: &impl TodoItemRepository) -> Result<()> {
        repo.create_table().context("✘ Couldn't create new table")
    }
}
