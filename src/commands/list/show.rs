use crate::domain::{Metadata, Status, TodoItem, TodoItemRepository, TodoList};
use anyhow::Result;
use colored::*;

impl TodoList {
    pub fn show(&self, repo: &impl TodoItemRepository, id: &str) -> Result<()> {
        let (item, metadata) = repo.fetch_item_and_metadata(id)?;
        pretty_print(item, metadata);
        Ok(())
    }
}

fn pretty_print(item: TodoItem, metadata: Metadata) {
    let status = match item.status {
        Status::Open => "open",
        Status::Closed => "done",
    };
    println!("Id: {}", item.id);
    println!("Created at: {}", metadata.created_at);
    println!("Last updated at: {}", metadata.last_updated);
    println!("Due by: {}", item.due);
    println!("Priority: {}", item.prio);
    println!("Status: {}", status);
    println!("Tag: {}", item.tag);
    println!("\n{}", item.task.blue().bold());
}
