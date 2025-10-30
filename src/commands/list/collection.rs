use std::error::Error;

use crate::queries::collection::Collection;
use crate::todo::TodoList;
use crate::util;

impl TodoList {
    pub fn list_collection(&self) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let collection = Collection::fetch_all(&conn)?;
        println!("Your collection\n===============");
        for list in collection.iter() {
            println!("• {list}");
        }
        Ok(())
    }
}
