use std::error::Error;

use crate::domain::TodoList;
use crate::persistence::table::Table;
use crate::util;

impl TodoList {
    pub fn delete_all(&mut self) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let current_list = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current_list);
        let table = Table {
            name: &current_list,
            conn: &conn,
        };
        let ids = table.fetch_all_ids()?;
        for id in ids.iter() {
            table.delete_by_id(*id)?;
        }
        Ok(())
    }
}
