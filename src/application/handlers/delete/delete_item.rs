use anyhow::Result;

use crate::domain::{TodoItemDelete, TodoList};

pub fn delete<R>(repo: &R, todo_list: &mut TodoList, id: &str) -> Result<()>
where
    R: TodoItemDelete,
{
    todo_list.delete_item(repo, id)?;
    Ok(())
}
