use anyhow::Result;

use crate::domain::{TodoList, TodoListRepository};

pub fn list_collection(repo: &impl TodoListRepository, todo_list: &TodoList) -> Result<()> {
    let collection = todo_list.get_collection(repo)?;
    println!("Your collection\n===============");
    for list in collection.iter() {
        println!("• {list}");
    }
    Ok(())
}
