use anyhow::Result;

use crate::domain::{Datetime, Prio, Status, Tag};
use crate::domain::{TodoItem, TodoItemRepository, TodoList};
use crate::persistence::schema::epoch;
use crate::util;

impl TodoList {
    pub fn add(
        &self,
        repo: &impl TodoItemRepository,
        flags: (Option<String>, Option<Datetime>, Option<Prio>, Option<Tag>),
    ) -> Result<()> {
        let (task, due, prio, tag) = flags;
        // logging
        match due.as_ref() {
            Some(date) => log::info!("found due date '{}'", date),
            None => log::info!("found due date 'None'"),
        };
        let msg = if let Some(task) = task {
            task
        } else {
            util::edit_in_editor(None)
        };
        log::info!("found task '{}'", msg);
        let mut item = TodoItem {
            id: String::new(),
            task: msg,
            due: due.unwrap_or(epoch()),
            status: Status::Open,
            tag: tag.unwrap_or_default(),
            prio: prio.unwrap_or_default(),
        };
        item.hash_id();
        repo.add(&item)?;
        Ok(())
    }
}
