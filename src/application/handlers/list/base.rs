use anyhow::{Context, Result};
use std::cmp::Reverse;

use crate::application::config::Config;
use crate::domain::ListFilter;
use crate::domain::{Datetime, TodoItem, TodoItemRepository, TodoList, TodoListTable};
use crate::infrastructure::{config, UserPaths};
use crate::util;

pub fn list(
    repo: &impl TodoItemRepository,
    todo_list: &TodoList,
    config: &Config,
    sort: Option<String>,
    filter: Option<ListFilter>,
) -> Result<()> {
    let current_list = std::env::var("CURRENT")?;
    log::debug!("found current list '{}'", &current_list,);
    let mut todos = todo_list.get_list(repo, filter)?;
    sort_tasks(&mut todos, sort)?;
    prettify(&mut todos);
    let table = TodoListTable::new(&todos, config);
    table.print();
    Ok(())
}

pub fn sort_tasks(todos: &mut [TodoItem], sort_key: Option<String>) -> Result<()> {
    let user_paths = UserPaths::new();
    let mut sort_key_default = config::read_config(&user_paths)
        .context("✘ Couldn't read config while retrieving the sort key")?
        .style
        .sort_by;
    if sort_key_default.is_empty() {
        sort_key_default = "id".to_string()
    };
    let sort_key = sort_key.as_deref().unwrap_or(&sort_key_default);
    log::debug!("using sort key {sort_key}");
    match sort_key {
        "id" => todos.sort_by_key(|entry| Reverse(entry.id.clone())),
        "prio" => todos.sort_by_key(|entry| entry.prio),
        "tag" => todos.sort_by_key(|entry| {
            let key = entry.tag.clone();
            (key.0.is_empty(), key)
        }),
        "due" => todos.sort_by_key(|entry| {
            let key = entry.due;
            (key == Datetime::epoch(), key)
        }),
        _ => todos.sort_by_key(|entry| Reverse(entry.id.clone())),
    };
    Ok(())
}

fn prettify(todos: &mut [TodoItem]) {
    for todo in todos.iter_mut() {
        todo.task = util::prettify(&todo.task);
    }
}
