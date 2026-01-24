use crate::domain::{Datetime, Prio, Tag, TodoItemRepository, TodoList};
use anyhow::Result;

pub fn clear(
    repo: &impl TodoItemRepository,
    todo_list: &TodoList,
    ids: Vec<String>,
    due: bool,
    prio: bool,
    tag: bool,
) -> Result<()> {
    let due = if due { Some(Datetime::epoch()) } else { None };
    let prio = if prio { Some(Prio::Empty) } else { None };
    let tag = if tag { Some(Tag::empty()) } else { None };
    todo_list.update_item(repo, due, prio, None, tag, ids)?;

    Ok(())
}
