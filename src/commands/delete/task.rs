use std::error::Error;

use crate::queries;
use crate::todo::TodoList;
use crate::util;

impl TodoList {
    pub fn delete(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
        let current = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current);
        let conn = util::connect_to_db(&self.db_path)?;
        log::debug!("executing querry {}", queries::delete_task(&current, id));
        conn.execute(&queries::delete_task(&current, id), [])?;
        Ok(())
    }
}
