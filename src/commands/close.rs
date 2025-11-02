use std::error::Error;

use crate::domain::Status;
use crate::domain::TodoList;
use crate::persistence::table::Table;
use crate::util;

impl TodoList {
    pub fn close(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
        let current = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current);
        let conn = util::connect_to_db(&self.db_path)?;
        let table = Table {
            name: &current,
            conn: &conn,
        };
        table.update_status(Status::Closed, id)?;
        Ok(())
    }
}
