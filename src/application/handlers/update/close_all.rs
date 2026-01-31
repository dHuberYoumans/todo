use anyhow::Result;

use crate::domain::{Prio, TodoItemUpdate, TodoList};

pub fn close_all<R>(repo: &R, todo_list: TodoList, prio: Option<Prio>) -> Result<()>
where
    R: TodoItemUpdate,
{
    todo_list.close_all(repo, prio)?;
    Ok(())
}
