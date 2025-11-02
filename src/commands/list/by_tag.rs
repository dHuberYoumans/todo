use std::error::Error;

use tabled::settings::{object::Columns, Modify, Style, Width};

use crate::domain::TodoList;
use crate::persistence::table::Table;
use crate::util;

impl TodoList {
    pub fn list_tag(&mut self, tag: String) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let current_list = std::env::var("CURRENT")?;
        let table = Table {
            name: &current_list,
            conn: &conn,
        };
        let clean_tag = tag.strip_prefix('#').unwrap_or(&tag);
        let entries = table.fetch_by_tag(clean_tag)?;
        for entry in entries {
            let _ = &self.tasks.push(entry);
        }
        let mut table = tabled::Table::new(&self.tasks);
        table
            .with(Modify::new(Columns::single(0)).with(Width::increase(5))) // id
            .with(Modify::new(Columns::single(1)).with(Width::wrap(60))) // task
            .with(Modify::new(Columns::single(2)).with(Width::increase(3))) // status
            .with(Modify::new(Columns::single(3)).with(Width::increase(3))) // prio
            .with(Modify::new(Columns::single(4)).with(Width::increase(3))) // due
            .with(Modify::new(Columns::single(5)).with(Width::wrap(12))) // tag
            .with(Style::modern_rounded());
        println!("{}", table);
        Ok(())
    }
}
