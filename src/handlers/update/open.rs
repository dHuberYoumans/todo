use anyhow::Result;

use crate::domain::Status;
use crate::domain::{TodoItemRepository, TodoList};

pub fn open(repo: &impl TodoItemRepository, todo_list: &TodoList, ids: Vec<String>) -> Result<()> {
    todo_list.update_item(repo, None, None, Some(Status::Open), None, ids)?;
    Ok(())
}
