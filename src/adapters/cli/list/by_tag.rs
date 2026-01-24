use anyhow::Result;

use crate::adapters::cli::sort_tasks;
use crate::commands::ListFilter;
use crate::domain::{Tag, TodoItem, TodoItemRepository, TodoList, TodoListTable};

pub fn list_tag(
    repo: &impl TodoItemRepository,
    todo_list: &TodoList,
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
    let table = TodoListTable::new(&tasks);
    table.print();
    Ok(())
}
