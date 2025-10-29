use std::error::Error;

use crate::queries;
use crate::todo::TodoList;
use crate::util;

impl TodoList {
    pub fn list_tags(&self) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let current = std::env::var("CURRENT")?;
        let mut stmt = conn.prepare(&queries::fetch_tags(&current))?;
        let tags_iter = stmt.query_map([], |row| {
            let list = row.get::<_, String>("tag")?;
            Ok(list)
        })?;
        println!("Your tags\n==========");
        for tag in tags_iter.flatten() {
            println!("• {tag}");
        }
        Ok(())
    }
}
