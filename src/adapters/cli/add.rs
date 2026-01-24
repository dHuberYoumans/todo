use anyhow::Result;
use uuid::Uuid;

use crate::domain::AddArgs;
use crate::domain::{Datetime, Status};
use crate::domain::{TodoItem, TodoItemRepository, TodoList};
use crate::util;

pub fn add(todo_list: &TodoList, repo: &impl TodoItemRepository, args: AddArgs) -> Result<()> {
    // logging
    match args.due.as_ref() {
        Some(date) => log::info!("found due date '{}'", date),
        None => log::info!("found due date 'None'"),
    };
    let msg = if let Some(task) = args.task {
        task
    } else {
        util::edit_in_editor(None)
    };
    log::info!("found task '{}'", msg);
    let item = TodoItem {
        id: Uuid::new_v4().to_string(),
        task: msg,
        due: args.due.unwrap_or(Datetime::epoch()),
        status: Status::Open,
        tag: args.tag.unwrap_or_default(),
        prio: args.prio.unwrap_or_default(),
    };
    todo_list.add_item(repo, &item)?;
    Ok(())
}
