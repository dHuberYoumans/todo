use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};

pub fn delete_all(repo: &impl TodoItemRepository, todo_list: &mut TodoList) -> Result<()> {
    println!("▶ Deleteing all todos...");
    todo_list.delete_all_items(repo)?;
    println!("✔ Done");
    Ok(())
}
