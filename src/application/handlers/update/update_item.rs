use anyhow::Result;

use crate::domain::{update::UpdateOptions, TodoItemUpdate, TodoList};

pub fn update_item<R>(
    repo: &R,
    todo_list: &TodoList,
    ids: Vec<String>,
    options: UpdateOptions,
) -> Result<()>
where
    R: TodoItemUpdate,
{
    todo_list.update_item(repo, ids, options)?;
    Ok(())
}
