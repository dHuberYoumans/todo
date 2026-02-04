use anyhow::{anyhow, Result};

use crate::domain::{TodoItem, TodoItemCreate, TodoList};

impl TodoList {
    pub fn add_item(&self, repo: &impl TodoItemCreate, item: &TodoItem) -> Result<()> {
        if item.task.is_empty() {
            Err(anyhow!("✘ Empty todo found"))
        } else {
            repo.add(item)
        }
    }
}
