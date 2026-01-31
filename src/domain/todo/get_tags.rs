use anyhow::{Context, Result};

use crate::domain::{Tag, TodoItemQueryColumns, TodoList};

impl TodoList {
    pub fn get_tags(&self, repo: &impl TodoItemQueryColumns) -> Result<Vec<Tag>> {
        repo.fetch_tags().context("✘ Couldn't fetch tags")
    }
}
