use std::error::Error;

use crate::queries::collection::Collection;
use crate::queries::table::Table;
use crate::todo::TodoList;
use crate::util;

impl TodoList {
    pub fn new_list(&mut self, list: String, checkout: bool) -> Result<(), Box<dyn Error>> {
        println!("⧖ Creating new_list..");
        let conn = util::connect_to_db(&self.db_path)?;
        println!("✔ Created '{list}' in collection");
        Collection::insert(&conn, &list)?;
        let new_list = Table {
            name: &list,
            conn: &conn,
        };
        new_list.create_table()?;
        println!("✔ Created new list '{list}'");
        if checkout {
            log::info!("checking out list '{list}'");
            self.load(list.clone())?;
            println!("✔ Now using '{list}'");
        };
        Ok(())
    }
}
