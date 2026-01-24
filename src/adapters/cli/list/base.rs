use anyhow::{Context, Result};
use std::cmp::Reverse;

use crate::adapters::cli::config;
use crate::domain::ListFilter;
use crate::domain::{Datetime, TodoItem, TodoItemRepository, TodoList, TodoListTable};

pub fn list(
    repo: &impl TodoItemRepository,
    todo_list: &TodoList,
    sort: Option<String>,
    filter: Option<ListFilter>,
) -> Result<()> {
    let current_list = std::env::var("CURRENT")?;
    log::debug!("found current list '{}'", &current_list,);
    let mut tasks = todo_list.get_list(repo, filter)?;
    sort_tasks(&mut tasks, sort)?;
    let table = TodoListTable::new(&tasks);
    table.print();
    Ok(())
}

pub fn sort_tasks(tasks: &mut [TodoItem], sort_key: Option<String>) -> Result<()> {
    let mut sort_key_default = config::fs::read()
        .context("✘ Couldn't read config while retrieving the sort key")?
        .style
        .sort_by;
    if sort_key_default.is_empty() {
        sort_key_default = "id".to_string()
    };
    let sort_key = sort_key.as_deref().unwrap_or(&sort_key_default);
    log::debug!("using sort key {sort_key}");
    match sort_key {
        "id" => tasks.sort_by_key(|entry| Reverse(entry.id.clone())),
        "prio" => tasks.sort_by_key(|entry| entry.prio),
        "tag" => tasks.sort_by_key(|entry| {
            let key = entry.tag.clone();
            (key.0.is_empty(), key)
        }),
        "due" => tasks.sort_by_key(|entry| {
            let key = entry.due;
            (key == Datetime::epoch(), key)
        }),
        _ => tasks.sort_by_key(|entry| Reverse(entry.id.clone())),
    };
    Ok(())
}
