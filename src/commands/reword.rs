use rusqlite::{params, OptionalExtension, Result};
use std::error::Error;

use crate::queries;
use crate::todo::TodoList;
use crate::util;

impl TodoList {
    pub fn reword(&mut self, input: (i64, Option<String>)) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let current_list = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current_list);
        let (id, task) = input;
        let msg = if let Some(task) = task {
            task
        } else {
            let mut stmt = conn.prepare(&queries::fetch_task_by_id(&current_list))?;
            let text: Option<String> = stmt
                .query_row(params![id], |row| row.get::<_, String>("task"))
                .optional()?;
            util::edit_in_editor(text)
        };
        log::info!("found task '{}'", &msg);
        log::debug!(
            "executing querry `{}` \n with params [{},{}]",
            &queries::unpdate_task_by_id(&current_list),
            &id,
            &msg
        );
        conn.execute(&queries::unpdate_task_by_id(&current_list), (&id, &msg))?;
        Ok(())
    }
}
