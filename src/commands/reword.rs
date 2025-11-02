use rusqlite::Result;
use std::error::Error;

use crate::domain::TodoList;
use crate::persistence::table::Table;
use crate::util;

impl TodoList {
    pub fn reword(&mut self, input: (i64, Option<String>)) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let current_list = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current_list);
        let table = Table {
            name: &current_list,
            conn: &conn,
        };
        let (id, task) = input;
        let msg = if let Some(task) = task {
            task
        } else {
            let text = table.fetch_task_by_id(id)?;
            util::edit_in_editor(text)
        };
        log::info!("found task '{}'", &msg);
        table.update_task_by_id(&msg, id)?;
        Ok(())
    }
}
