use anyhow::{Context, Result};

use crate::commands::ListFilter;
use crate::domain::{Tag, TodoItem, TodoItemRepository, TodoList};

impl TodoList {
    pub fn get_entries_by_tag(
        &self,
        repo: &impl TodoItemRepository,
        tag: Tag,
        filter: Option<ListFilter>,
    ) -> Result<Vec<TodoItem>> {
        repo.fetch_by_tag(tag, filter)
            .context("✘ Couldn't fetch entries")
    }
}
