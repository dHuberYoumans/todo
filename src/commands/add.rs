use rusqlite::Result;
use std::error::Error;

use crate::domain::TodoList;
use crate::domain::{Datetime, Prio, Status, Tag};
use crate::persistence::schema::epoch;
use crate::persistence::Table;
use crate::util;

impl TodoList {
    pub fn add(
        &mut self,
        flags: (Option<String>, Option<Prio>, Option<String>, Option<String>),
    ) -> Result<(), Box<dyn Error>> {
        let current_list = std::env::var("CURRENT")?;
        log::info!("currently on list {current_list}");
        let conn = util::connect_to_db(&self.db_path)?;
        let table = Table {
            name: &current_list,
            conn: &conn,
        };
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
        table.insert(
            &msg,
            Status::Open,
            prio.unwrap_or_default(),
            due_date.as_ref().unwrap_or(&epoch()),
            &tag.unwrap_or_default(),
            &Datetime::now(),
        )?;
        Ok(())
    }
}
