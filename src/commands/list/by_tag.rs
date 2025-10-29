use std::error::Error;

use tabled::{
    settings::{object::Columns, Modify, Style, Width},
    Table,
};

use crate::queries;
use crate::todo::TodoList;
use crate::util::{self, Datetime, Prio, Status, Tag, TodoItem};

impl TodoList {
    pub fn list_tag(&mut self, tag: String) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        log::debug!("executing query `{}`", &queries::fetch_tag(tag.as_str()));
        let mut stmt = conn.prepare(&queries::fetch_tag(tag.as_str()))?;
        let entries = stmt.query_map([], |row| {
            Ok(TodoItem {
                id: row.get::<_, i64>("id")?,
                task: row.get::<_, String>("task")?,
                status: row.get::<_, Status>("status")?,
                prio: row.get::<_, Prio>("prio")?,
                due: row.get::<_, Datetime>("due")?,
                tag: row.get::<_, Tag>("tag")?,
            })
        })?;
        for entry in entries {
            let _ = &self.tasks.push(entry?);
        }
        let mut table = Table::new(&self.tasks);
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
