use anyhow::Result;

use crate::domain::{Datetime, Prio, Status, Tag, TodoItemRepository, TodoList};

pub fn update_item(
    repo: &impl TodoItemRepository,
    todo_list: &TodoList,
    due: Option<Datetime>,
    prio: Option<Prio>,
    status: Option<Status>,
    tag: Option<Tag>,
    ids: Vec<String>,
) -> Result<()> {
    todo_list.update_item(repo, due, prio, status, tag, ids)?;
    Ok(())
}
