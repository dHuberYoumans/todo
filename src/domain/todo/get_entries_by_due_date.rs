use anyhow::{Context, Result};

use crate::commands::ListFilter;
use crate::domain::{TodoItem, TodoItemRepository, TodoList};

impl TodoList {
    pub fn get_entries_by_due_date(
        &self,
        repo: &impl TodoItemRepository,
        epoch_seconds: i64,
        filter: Option<ListFilter>,
    ) -> Result<Vec<TodoItem>> {
        repo.fetch_by_due_date(epoch_seconds, filter)
            .context("✘ Couldn't fetch entries")
    }
}
