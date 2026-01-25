use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};

pub fn list_tags(repo: &impl TodoItemRepository, todo_list: &TodoList) -> Result<()> {
    let tags = todo_list.get_tags(repo)?;
    println!("Your tags\n==========");
    for tag in tags.iter() {
        println!("• {tag}");
    }
    Ok(())
}
