use anyhow::Result;

use crate::domain::{Datetime, Prio, Status, Tag, TodoItemUpdate, TodoList};

impl TodoList {
    pub fn update_item(
        &self,
        repo: &impl TodoItemUpdate,
        due: Option<Datetime>,
        prio: Option<Prio>,
        status: Option<Status>,
        tag: Option<Tag>,
        ids: Vec<String>,
    ) -> Result<()> {
        repo.update(due, prio, status, tag, ids)?;
        Ok(())
    }
}
