use anyhow::Result;

use crate::domain::{update::UpdateOptions, Status, TodoItemUpdate, TodoList};

pub fn close<R>(repo: &R, todo_list: &TodoList, ids: Vec<String>) -> Result<()>
where
    R: TodoItemUpdate,
{
    let options = UpdateOptions {
        due: None,
        prio: None,
        status: Some(Status::Closed),
        tag: None,
    };
    todo_list.update_item(repo, ids, options)?;
    Ok(())
}
