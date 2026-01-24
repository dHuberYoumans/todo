use anyhow::Result;

use crate::domain::{Status, TodoItemRepository, TodoList};

pub fn close(repo: &impl TodoItemRepository, todo_list: &TodoList, ids: Vec<String>) -> Result<()> {
    todo_list.update_item(repo, None, None, Some(Status::Closed), None, ids)?;
    Ok(())
}
