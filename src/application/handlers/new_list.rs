use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList, TodoListRepository};

pub fn new_list(
    todo_list_repo: &impl TodoListRepository,
    todo_item_repo: &impl TodoItemRepository,
    todo_list: &TodoList,
    list: &str,
) -> Result<()> {
    println!("▶ Creating new list '{list}'...");
    todo_list.add_list(todo_list_repo, list)?;
    todo_list.create_table(todo_item_repo)?;
    println!("✔ Done");
    Ok(())
}
