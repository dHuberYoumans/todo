use anyhow::Result;
use uuid::Uuid;

use crate::application::editor::Editor;
use crate::domain::{add_item::AddOptions, Datetime, Status, TodoItem, TodoItemCreate, TodoList};

pub fn add<R>(repo: &R, todo_list: &TodoList, editor: &impl Editor, args: AddOptions) -> Result<()>
where
    R: TodoItemCreate,
{
    // logging
    match args.due.as_ref() {
        Some(date) => log::info!("found due date '{}'", date),
        None => log::info!("found due date 'None'"),
    };
    let msg = if let Some(task) = args.task {
        task
    } else {
        editor.edit(None)?
    };
    log::info!("found task '{}'", msg);
    let due = args.due.unwrap_or(Datetime::epoch());
    let item = TodoItem {
        id: Uuid::new_v4().to_string(),
        task: msg,
        due,
        status: Status::Open,
        tag: args.tag.unwrap_or_default(),
        prio: args.prio.unwrap_or_default(),
    };
    todo_list.add_item(repo, &item)?;
    Ok(())
}
