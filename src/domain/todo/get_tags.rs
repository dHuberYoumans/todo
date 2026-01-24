use anyhow::{Context, Result};

use crate::domain::{Tag, TodoItemRepository, TodoList};

impl TodoList {
    pub fn get_tags(&self, repo: &impl TodoItemRepository) -> Result<Vec<Tag>> {
        repo.fetch_tags().context("✘ Couldn't fetch tags")
    }
}
