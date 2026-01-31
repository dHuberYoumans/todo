use anyhow::Result;

use crate::domain::{TodoItemDelete, TodoList};

pub fn delete_all<R>(repo: &R, todo_list: &mut TodoList) -> Result<()>
where
    R: TodoItemDelete,
{
    println!("▶ Deleteing all todos...");
    todo_list.delete_all_items(repo)?;
    println!("✔ Done");
    Ok(())
}
