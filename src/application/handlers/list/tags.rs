use anyhow::Result;

use crate::domain::{TodoItemQueryColumns, TodoList};

pub fn list_tags<R>(repo: &R, todo_list: &TodoList) -> Result<()>
where
    R: TodoItemQueryColumns,
{
    let tags = todo_list.get_tags(repo)?;
    println!("Your tags\n==========");
    for tag in tags.iter() {
        println!("• {tag}");
    }
    Ok(())
}
