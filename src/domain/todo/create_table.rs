use anyhow::{Context, Result};

use crate::domain::{TodoItemSchema, TodoList};

impl TodoList {
    pub fn create_table(&self, repo: &impl TodoItemSchema) -> Result<()> {
        repo.create_table().context("✘ Couldn't create new table")
    }
}
