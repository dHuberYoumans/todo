use anyhow::{anyhow, Result};

use crate::application::config::Config;
use crate::domain::ListFilter;
use crate::domain::{Datetime, TodoItem, TodoItemRepository, TodoList, TodoListTable};
use crate::handlers::sort_tasks;

pub fn list_due_date(
    repo: &impl TodoItemRepository,
    todo_list: &TodoList,
    config: &Config,
    date_str: String,
    sort: Option<String>,
    filter: Option<ListFilter>,
) -> Result<()> {
    let epoch_seconds = if let Some(date) = date_str.strip_prefix("@") {
        Datetime::parse(date, config.style.due_date_input_format.clone())?.timestamp
    } else {
        return Err(anyhow!("✘ Invalid date"));
    };
    let entries = todo_list.get_entries_by_due_date(repo, epoch_seconds, filter)?;
    let mut tasks: Vec<TodoItem> = Vec::new();
    for entry in entries {
        tasks.push(entry);
    }
    sort_tasks(&mut tasks, sort)?;
    let table = TodoListTable::new(&tasks, config);
    table.print();
    Ok(())
}
