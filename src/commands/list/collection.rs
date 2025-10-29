use std::error::Error;

use crate::queries;
use crate::todo::TodoList;
use crate::util;

impl TodoList {
    pub fn list_collection(&self) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let mut stmt = conn.prepare(&queries::fetch_collection())?;
        let collection_iter = stmt.query_map([], |row| {
            let list = row.get::<_, String>("name")?;
            Ok(list)
        })?;
        println!("Your collection\n===============");
        for list in collection_iter.flatten() {
            println!("• {list}");
        }
        Ok(())
    }
}
