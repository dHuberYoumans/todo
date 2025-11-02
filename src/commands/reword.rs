use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};
use crate::util;

impl TodoList {
    pub fn reword(
        &mut self,
        repo: &impl TodoItemRepository,
        id: i64,
        task: Option<String>,
    ) -> Result<()> {
        let msg = if let Some(task) = task {
            task
        } else {
            let text = repo.fetch_task_by_id(id)?;
            util::edit_in_editor(text)
        };
        log::info!("found task '{}'", &msg);
        repo.update_task(&msg, id)?;
        Ok(())
    }
}
