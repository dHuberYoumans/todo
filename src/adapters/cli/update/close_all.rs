use anyhow::Result;

use crate::domain::{Prio, TodoItemRepository, TodoList};

pub fn close_all(
    repo: &impl TodoItemRepository,
    todo_list: TodoList,
    prio: Option<Prio>,
) -> Result<()> {
    todo_list.close_all(repo, prio)?;
    Ok(())
}
