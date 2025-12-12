use crate::domain::{Metadata, Status, TodoItem, TodoItemRepository, TodoList};
use anyhow::Result;
use colored::*;

use crate::util;

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
    let (title, message) = util::parse_task(&item.task);
    println!("Id: {}", item.id);
    println!("Created at: {}", metadata.created_at);
    println!("Last updated at: {}", metadata.last_updated);
    println!("Due by: {}", item.due);
    println!("Priority: {}", item.prio);
    println!("Status: {}", status);
    println!("Tag: {}", item.tag);
    println!("\n{}", title.magenta().bold());
    println!();
    println!("{}", message);
}
