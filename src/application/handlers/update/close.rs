use anyhow::Result;

use crate::domain::{Status, TodoItemUpdate, TodoList};

pub fn close<R>(repo: &R, todo_list: &TodoList, ids: Vec<String>) -> Result<()>
where
    R: TodoItemUpdate,
{
    todo_list.update_item(repo, None, None, Some(Status::Closed), None, ids)?;
    Ok(())
}
