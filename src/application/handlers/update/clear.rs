use crate::domain::{
    update::{ClearOptions, UpdateOptions},
    Datetime, Prio, Tag, TodoItemUpdate, TodoList,
};
use anyhow::Result;

pub fn clear<R>(
    repo: &R,
    todo_list: &TodoList,
    ids: Vec<String>,
    options: ClearOptions,
) -> Result<()>
where
    R: TodoItemUpdate,
{
    let due = if options.due {
        Some(Datetime::epoch())
    } else {
        None
    };
    let prio = if options.prio {
        Some(Prio::Empty)
    } else {
        None
    };
    let tag = if options.tag {
        Some(Tag::empty())
    } else {
        None
    };
    let options = UpdateOptions {
        due,
        prio,
        status: None,
        tag,
    };
    todo_list.update_item(repo, ids, options)?;
    Ok(())
}
