use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList};

impl TodoList {
    pub fn list_tags(&self, repo: &impl TodoItemRepository) -> Result<()> {
        let tags = repo.fetch_tags()?;
        println!("Your tags\n==========");
        for tag in tags.iter() {
            println!("• {tag}");
        }
        Ok(())
    }
}
