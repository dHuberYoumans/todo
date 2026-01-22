use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};
use crate::util;

pub fn reword(
    repo: &impl TodoItemRepository,
    todo_list: &mut TodoList,
    id: &str,
    task: Option<String>,
) -> Result<()> {
    let msg = if let Some(task) = task {
        task
    } else {
        let text = todo_list.get_entry(repo, id)?;
        util::edit_in_editor(text)
    };
    log::info!("found task '{}'", &msg);
    todo_list.update_task(repo, &msg, id)?;
    Ok(())
}
