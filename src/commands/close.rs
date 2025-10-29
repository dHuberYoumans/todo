use std::error::Error;

use crate::queries;
use crate::todo::TodoList;
use crate::util::{self, Status};

impl TodoList {
    pub fn close(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
        let current = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current);
        let conn = util::connect_to_db(&self.db_path)?;
        log::debug!(
            "executing querry `{}` \n with params [{},{}]",
            &queries::update_status(&current),
            &Status::Closed,
            &id
        );
        conn.execute(&queries::update_status(&current), (&Status::Closed, &id))?;
        Ok(())
    }
}
