use anyhow::{anyhow, Result};

use crate::domain::{Datetime, TodoItemRepository, TodoList, TodoListTable};

impl TodoList {
    pub fn list_due_date(
        &mut self,
        repo: &impl TodoItemRepository,
        date_str: String,
    ) -> Result<()> {
        let epoch_seconds = if let Some(date) = date_str.strip_prefix("@") {
            Datetime::parse(date)?.timestamp
        } else {
            return Err(anyhow!("✘ Invalid date"));
        };
        let entries = repo.fetch_by_due_date(epoch_seconds)?;
        for entry in entries {
            let _ = &self.tasks.push(entry);
        }
        let table = TodoListTable::new(&self.tasks);
        table.print();
        Ok(())
    }
}
