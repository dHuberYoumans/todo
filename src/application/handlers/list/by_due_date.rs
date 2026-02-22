use anyhow::{anyhow, Result};

use crate::application::config::Config;
use crate::application::handlers::sort_tasks;
use crate::domain::{
    Datetime, ListFilters, StatusFilter, TodoItem, TodoItemQuery, TodoList, TodoListTable,
};

pub fn list_due_date<R>(
    repo: &R,
    todo_list: &TodoList,
    config: &Config,
    date_str: String,
    sort: Option<String>,
    filters: ListFilters,
) -> Result<()>
where
    R: TodoItemQuery,
{
    let epoch_seconds = if let Some(date) = date_str.strip_prefix("@") {
        Datetime::parse(date, config.style.due_date_input_format.clone())?.timestamp
    } else {
        return Err(anyhow!("✘ Invalid date"));
    };
    let filters_or_default = ListFilters {
        status: Some(filters.status.unwrap_or(StatusFilter::Do)),
        prio: filters.prio,
    };
    let entries = todo_list.get_entries_by_due_date(repo, epoch_seconds, filters_or_default)?;
    let mut tasks: Vec<TodoItem> = Vec::new();
    for entry in entries {
        tasks.push(entry);
    }
    sort_tasks(&mut tasks, sort)?;
    let table = TodoListTable::new(&tasks, config);
    table.print();
    Ok(())
}
