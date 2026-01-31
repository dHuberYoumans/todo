use anyhow::Result;

use crate::domain::Status;
use crate::domain::{TodoItemUpdate, TodoList};

pub fn open<R>(repo: &R, todo_list: &TodoList, ids: Vec<String>) -> Result<()>
where
    R: TodoItemUpdate,
{
    todo_list.update_item(repo, None, None, Some(Status::Open), None, ids)?;
    Ok(())
}
