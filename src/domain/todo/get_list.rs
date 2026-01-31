use anyhow::{Context, Result};

use crate::domain::ListFilter;
use crate::domain::{TodoItem, TodoItemRead, TodoList};

impl TodoList {
    pub fn get_list(
        &self,
        repo: &impl TodoItemRead,
        filter: Option<ListFilter>,
    ) -> Result<Vec<TodoItem>> {
        repo.fetch_list(filter).context("✘ Couldn't fetch todos")
    }
}
