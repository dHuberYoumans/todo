use anyhow::{Context, Result};

use crate::domain::ListFilter;
use crate::domain::{TodoItem, TodoItemRepository, TodoList};

impl TodoList {
    pub fn get_list(
        &self,
        repo: &impl TodoItemRepository,
        filter: Option<ListFilter>,
    ) -> Result<Vec<TodoItem>> {
        repo.fetch_list(filter).context("✘ Couldn't fetch todos")
    }
}
