use anyhow::Result;

use crate::application::editor::Editor;
use crate::domain::{TodoItemRepository, TodoList};

pub fn reword(
    repo: &impl TodoItemRepository,
    todo_list: &mut TodoList,
    editor: &impl Editor,
    id: &str,
    task: Option<String>,
) -> Result<()> {
    let msg = if let Some(task) = task {
        task
    } else {
        let text = todo_list.get_entry(repo, id)?;
        editor.edit(text)?
    };
    log::info!("found task '{}'", &msg);
    todo_list.update_task(repo, &msg, id)?;
    Ok(())
}
