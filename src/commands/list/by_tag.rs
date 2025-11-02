use anyhow::Result;

use tabled::settings::{object::Columns, Modify, Style, Width};

use crate::domain::Tag;
use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn list_tag(&mut self, repo: &impl TodoItemRepository, tag: String) -> Result<()> {
        let clean_tag = tag.strip_prefix('#').unwrap_or(&tag);
        let entries = repo.fetch_by_tag(Tag(clean_tag.to_string()))?;
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
