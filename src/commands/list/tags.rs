use std::error::Error;

use crate::domain::TodoList;
use crate::persistence::table::Table;
use crate::util;

impl TodoList {
    pub fn list_tags(&self) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let current_list = std::env::var("CURRENT")?;
        let table = Table {
            name: &current_list,
            conn: &conn,
        };
        let tags = table.fetch_tags()?;
        println!("Your tags\n==========");
        for tag in tags.iter() {
            println!("• {tag}");
        }
        Ok(())
    }
}
