use anyhow::Result;

use crate::domain::{Tag, TodoListTable};
use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn list_tag(&mut self, repo: &impl TodoItemRepository, tag: String) -> Result<()> {
        let clean_tag = tag.strip_prefix('#').unwrap_or(&tag);
        let entries = repo.fetch_by_tag(Tag(clean_tag.to_string()))?;
        for entry in entries {
            let _ = &self.tasks.push(entry);
        }
        let table = TodoListTable::new(&self.tasks);
        table.print();
        Ok(())
    }
}
