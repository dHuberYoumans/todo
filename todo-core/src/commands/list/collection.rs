use anyhow::Result;

use crate::domain::{TodoList, TodoListRepository};

impl TodoList {
    pub fn list_collection(&self, repo: &impl TodoListRepository) -> Result<()> {
        let collection = repo.fetch_all()?;
        println!("Your collection\n===============");
        for list in collection.iter() {
            println!("• {list}");
        }
        Ok(())
    }
}
