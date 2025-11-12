use anyhow::Result;

use crate::domain::{Datetime, Prio, Status, Tag};
use crate::domain::{TodoItem, TodoItemRepository, TodoList};
use crate::persistence::schema::epoch;
use crate::util;

impl TodoList {
    pub fn add(
        &self,
        repo: &impl TodoItemRepository,
        flags: (Option<String>, Option<Prio>, Option<String>, Option<String>),
    ) -> Result<()> {
        let (task, prio, due, raw_tag) = flags;
        let tag: Option<Tag> = raw_tag.map(Tag);
        let due_date: Option<Datetime> = match due {
            Some(ref date) => Some(util::parse_date(date)?),
            None => None,
        };
        // logging
        match due_date.as_ref() {
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
            due: *due_date.as_ref().unwrap_or(&epoch()),
            status: Status::Open,
            tag: tag.unwrap_or_default(),
            prio: prio.unwrap_or_default(),
        };
        item.hash_id();
        repo.add(&item)?;
        Ok(())
    }
}
