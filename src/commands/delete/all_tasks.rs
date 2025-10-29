use std::error::Error;

use crate::queries;
use crate::todo::TodoList;
use crate::util;

impl TodoList {
    pub fn delete_all(&mut self) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let current_list = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current_list);
        log::debug!(
            "executing querry `{}`",
            &queries::fetch_all_ids(&current_list)
        );
        let mut stmt = conn.prepare(&queries::fetch_all_ids(&current_list))?;
        let ids_iter = stmt.query_map([], |row| {
            let id = row.get::<_, i64>("id")?;
            Ok(id)
        })?;
        for id in ids_iter {
            log::debug!(
                "executing querry `{}` \n with params [{}]",
                &queries::delete_by_id(&current_list),
                id.as_ref().unwrap()
            );
            conn.execute(&queries::delete_by_id(&current_list), [&id.unwrap()])?;
        }
        Ok(())
    }
}
