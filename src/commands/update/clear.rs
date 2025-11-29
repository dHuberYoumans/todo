use crate::domain::{Datetime, Prio, Tag, TodoItemRepository, TodoList};
use anyhow::Result;

impl TodoList {
    pub fn clear(
        &self,
        repo: &impl TodoItemRepository,
        ids: Vec<String>,
        due: bool,
        prio: bool,
        tag: bool,
    ) -> Result<()> {
        let due = if due { Some(Datetime::epoch()) } else { None };
        let prio = if prio { Some(Prio::Empty) } else { None };
        let tag = if tag { Some(Tag::empty()) } else { None };
        repo.update(due, prio, None, tag, ids)?;

        Ok(())
    }
}
