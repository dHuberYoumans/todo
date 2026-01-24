use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};

pub fn delete(repo: &impl TodoItemRepository, todo_list: &mut TodoList, id: &str) -> Result<()> {
    todo_list.delete_item(repo, id)?;
    Ok(())
}
