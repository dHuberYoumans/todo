use anyhow::Result;
use uuid::Uuid;

use crate::application::config::Config;
use crate::application::editor::Editor;
use crate::domain::AddArgs;
use crate::domain::{Datetime, Status};
use crate::domain::{TodoItem, TodoItemRepository, TodoList};

pub fn add(
    repo: &impl TodoItemRepository,
    todo_list: &TodoList,
    config: &Config,
    editor: &impl Editor,
    args: AddArgs,
) -> Result<()> {
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
    let due = args
        .due
        .as_deref()
        .map(|s| Datetime::parse(s, config.style.due_date_input_format.clone()))
        .transpose()?;
    let item = TodoItem {
        id: Uuid::new_v4().to_string(),
        task: msg,
        due: due.unwrap_or(Datetime::epoch()),
        status: Status::Open,
        tag: args.tag.unwrap_or_default(),
        prio: args.prio.unwrap_or_default(),
    };
    todo_list.add_item(repo, &item)?;
    Ok(())
}
