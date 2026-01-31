use anyhow::Result;

use crate::domain::{Datetime, Prio, Status, Tag, TodoItemUpdate, TodoList};

pub fn update_item<R>(
    repo: &R,
    todo_list: &TodoList,
    due: Option<Datetime>,
    prio: Option<Prio>,
    status: Option<Status>,
    tag: Option<Tag>,
    ids: Vec<String>,
) -> Result<()>
where
    R: TodoItemUpdate,
{
    todo_list.update_item(repo, due, prio, status, tag, ids)?;
    Ok(())
}
