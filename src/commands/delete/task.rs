use std::error::Error;

use crate::domain::TodoList;
use crate::persistence::table::Table;
use crate::util;

impl TodoList {
    pub fn delete(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
        let current_list = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current_list);
        let conn = util::connect_to_db(&self.db_path)?;
        let table = Table {
            name: &current_list,
            conn: &conn,
        };
        table.delete_task(id)?;
        Ok(())
    }
}
