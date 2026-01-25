use anyhow::Result;

use crate::application::config::Config;
use crate::application::handlers::sort_tasks;
use crate::domain::ListFilter;
use crate::domain::{Tag, TodoItem, TodoItemRepository, TodoList, TodoListTable};

pub fn list_tag(
    repo: &impl TodoItemRepository,
    todo_list: &TodoList,
    config: &Config,
    tag: String,
    sort: Option<String>,
    filter: Option<ListFilter>,
) -> Result<()> {
    let clean_tag = tag.strip_prefix('#').unwrap_or(&tag);
    let entries = todo_list.get_entries_by_tag(repo, Tag(clean_tag.to_string()), filter)?;
    let mut tasks: Vec<TodoItem> = Vec::new();
    for entry in entries {
        tasks.push(entry);
    }
    sort_tasks(&mut tasks, sort)?;
    let table = TodoListTable::new(&tasks, config);
    table.print();
    Ok(())
}
