use anyhow::{Context, Result};

use crate::domain::{ListFilter, TodoItem, TodoItemQuery, TodoList};

impl TodoList {
    pub fn get_entries_by_due_date(
        &self,
        repo: &impl TodoItemQuery,
        epoch_seconds: i64,
        filter: Option<ListFilter>,
    ) -> Result<Vec<TodoItem>> {
        repo.fetch_by_due_date(epoch_seconds, filter)
            .context("✘ Couldn't fetch entries")
    }
}
