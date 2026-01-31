use anyhow::Result;

use crate::domain::{TodoItemSchema, TodoList, TodoListCreate};

pub fn new_list<L, I>(
    todo_list_repo: &L,
    todo_item_repo: &I,
    todo_list: &TodoList,
    list: &str,
) -> Result<()>
where
    L: TodoListCreate,
    I: TodoItemSchema,
{
    println!("▶ Creating new list '{list}'...");
    todo_list.add_list(todo_list_repo, list)?;
    todo_list.create_table(todo_item_repo)?;
    println!("✔ Done");
    Ok(())
}
