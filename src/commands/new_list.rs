use std::error::Error;

use crate::queries;
use crate::todo::TodoList;
use crate::util;

impl TodoList {
    pub fn new_list(&mut self, list: String, checkout: bool) -> Result<(), Box<dyn Error>> {
        println!("⧖ Creating new_list..");
        let conn = util::connect_to_db(&self.db_path)?;
        log::debug!("executing query `{}`", &queries::create_list(&list));
        conn.execute(&queries::create_list(&list), [])?;
        println!("✔ Created new list '{list}'");
        log::debug!("executing query `{}`", &queries::add_to_collection(&list));
        conn.execute(&queries::add_to_collection(&list), [])?;
        println!("✔ Added '{list}' to collection");
        if checkout {
            log::info!("checking out list '{list}'");
            self.load(list.clone())?;
            println!("✔ Now using '{list}'");
        };
        Ok(())
    }
}
