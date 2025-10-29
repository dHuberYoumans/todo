use rusqlite::{Result, ToSql};
use std::error::Error;

use crate::queries;
use crate::todo::TodoList;
use crate::util::{self, epoch, Datetime, Status, Tag};

impl TodoList {
    pub fn add(
        &mut self,
        flags: (Option<String>, Option<i64>, Option<String>, Option<String>),
    ) -> Result<(), Box<dyn Error>> {
        let current_list = std::env::var("CURRENT")?;
        log::info!("currently on list {current_list}");
        let current_list_id = util::fetch_active_list_id(&self.db_path)?;
        let conn = util::connect_to_db(&self.db_path)?;
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
        log::debug!(
            "executing querry `{}`\n with params [{},{},{},{},{}]",
            &queries::add_to_table(&current_list, current_list_id),
            &msg,
            &Status::Open,
            &prio.unwrap_or_default(),
            &due_date.as_ref().unwrap_or(&epoch()),
            &Datetime::new()
        );
        conn.execute(
            &queries::add_to_table(&current_list, current_list_id),
            [
                &msg,
                &Status::Open as &dyn ToSql,
                &prio.unwrap_or_default() as &dyn ToSql,
                &due_date.unwrap_or(epoch()),
                &tag,
                &Datetime::new() as &dyn ToSql,
            ],
        )?;
        Ok(())
    }
}
