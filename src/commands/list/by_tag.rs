use anyhow::Result;

use crate::commands::list::base::sort_tasks;
use crate::commands::ListFilter;
use crate::domain::{Tag, TodoListTable};
use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn list_tag(
        &mut self,
        repo: &impl TodoItemRepository,
        tag: String,
        sort: Option<String>,
        filter: Option<ListFilter>,
    ) -> Result<()> {
        let clean_tag = tag.strip_prefix('#').unwrap_or(&tag);
        let entries = repo.fetch_by_tag(Tag(clean_tag.to_string()), filter)?;
        for entry in entries {
            let _ = &self.tasks.push(entry);
        }
        sort_tasks(&mut self.tasks, sort)?;
        let table = TodoListTable::new(&self.tasks);
        table.print();
        Ok(())
    }
}
