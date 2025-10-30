use std::error::Error;

use crate::queries::schema::Status;
use crate::queries::table::Table;
use crate::todo::TodoList;
use crate::util;

impl TodoList {
    pub fn open(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
        let current = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current);
        let conn = util::connect_to_db(&self.db_path)?;
        let table = Table {
            name: &current,
            conn: &conn,
        };
        table.update_status(Status::Open, id)?;
        Ok(())
    }
}
