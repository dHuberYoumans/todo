use anyhow::{Context, Result};

use crate::domain::{ListFilter, Tag, TodoItem, TodoItemQuery, TodoList};

impl TodoList {
    pub fn get_entries_by_tag(
        &self,
        repo: &impl TodoItemQuery,
        tag: Tag,
        filter: Option<ListFilter>,
    ) -> Result<Vec<TodoItem>> {
        repo.fetch_by_tag(tag, filter)
            .context("✘ Couldn't fetch entries")
    }
}
