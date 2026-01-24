use anyhow::{Context, Result};

use crate::domain::{Metadata, TodoItem, TodoItemRepository, TodoList};

impl TodoList {
    pub fn get_entry_with_metadata(
        &self,
        repo: &impl TodoItemRepository,
        id: &str,
    ) -> Result<(TodoItem, Metadata)> {
        repo.fetch_item_and_metadata(id)
            .context("✘ Couldn't fetch entry")
    }
}
