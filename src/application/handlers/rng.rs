use anyhow::Result;

use crate::domain::{TodoItem, TodoItemRead, TodoList};

pub fn rnd<R>(repo: &R, todo_list: &TodoList) -> Result<Option<TodoItem>>
where
    R: TodoItemRead,
{
    let rnd_todo = todo_list.get_rnd_item(repo)?;
    if rnd_todo.is_none() {
        println!("✘ Didn't find any items with priority 'RND'.\n▶ Please update the priority of some of your todos to 'RND' so that I can suggest you a random todo among them.");
        Ok(None)
    } else {
        Ok(rnd_todo)
    }
}
