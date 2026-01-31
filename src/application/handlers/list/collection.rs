use anyhow::Result;

use crate::domain::{TodoList, TodoListRead};

pub fn list_collection<R>(repo: &R, todo_list: &TodoList) -> Result<()>
where
    R: TodoListRead,
{
    let collection = todo_list.get_collection(repo)?;
    println!("Your collection\n===============");
    for list in collection.iter() {
        println!("• {list}");
    }
    Ok(())
}
