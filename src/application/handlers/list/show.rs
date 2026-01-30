use crate::domain::{Metadata, Status, TodoItem, TodoItemRepository, TodoList};
use anyhow::Result;
use colored::*;

use crate::util;

pub fn show(repo: &impl TodoItemRepository, todo_list: &TodoList, id: &str) -> Result<()> {
    let (item, metadata) = todo_list.get_entry_with_metadata(repo, id)?;
    pretty_print(item, metadata);
    Ok(())
}

fn pretty_print(item: TodoItem, metadata: Metadata) {
    let status = match item.status {
        Status::Open => "open",
        Status::Closed => "done",
    };
    let (title, message) = util::parse_task(&item.task);
    let title = util::prettify(&title);
    let message = util::prettify(&message);
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
