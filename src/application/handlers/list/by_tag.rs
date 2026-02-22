use anyhow::Result;

use crate::application::config::Config;
use crate::application::handlers::sort_tasks;
use crate::domain::{
    ListFilters, StatusFilter, Tag, TodoItem, TodoItemQuery, TodoList, TodoListTable,
};

pub fn list_tag<R>(
    repo: &R,
    todo_list: &TodoList,
    config: &Config,
    tag: String,
    sort: Option<String>,
    filters: ListFilters,
) -> Result<()>
where
    R: TodoItemQuery,
{
    let clean_tag = tag.strip_prefix('#').unwrap_or(&tag);
    let filters_or_default = ListFilters {
        status: Some(filters.status.unwrap_or(StatusFilter::Do)),
        prio: filters.prio,
        due: filters.due,
        tag: None,
    };
    let entries =
        todo_list.get_entries_by_tag(repo, Tag(clean_tag.to_string()), filters_or_default)?;
    let mut tasks: Vec<TodoItem> = Vec::new();
    for entry in entries {
        tasks.push(entry);
    }
    sort_tasks(&mut tasks, sort)?;
    let table = TodoListTable::new(&tasks, config);
    table.print();
    Ok(())
}
