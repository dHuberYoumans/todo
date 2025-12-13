use clap::Parser;
use log;
use std::path::PathBuf;
use tabled::Tabled;

use crate::commands::Cmd;
use crate::domain::{Datetime, Prio, Status, Tag};
use crate::paths::UserPaths;
use crate::util::parse_task;

#[derive(Parser, Debug)]
#[command(
    name = "todo",
    version,
    about = "A simple todo cli to help you get things done from the comfort of your terminal"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Cmd>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct TodoList {
    pub tasks: Vec<TodoItem>,
    pub db_path: PathBuf,
}

impl TodoList {
    pub fn new() -> Self {
        log::debug!("instantiating new 'todo' struct");
        let user_paths = UserPaths::new();
        let db_path = user_paths
            .get_db()
            .expect("✘ No path to database found. Consider 'todo init' to initialize a data base");
        Self {
            tasks: Vec::new(),
            db_path,
        }
    }
}

impl Default for TodoList {
    fn default() -> Self {
        TodoList::new()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct TodoItem {
    pub id: String,
    pub task: String,
    pub status: Status,
    pub prio: Prio,
    pub due: Datetime,
    pub tag: Tag,
}

#[derive(Tabled)]
pub struct TodoItemRow {
    pub id: String,
    pub title: String,
    pub status: Status,
    pub prio: Prio,
    pub due: Datetime,
    pub tag: Tag,
}

impl From<&TodoItem> for TodoItemRow {
    fn from(item: &TodoItem) -> Self {
        let (title, _message) = parse_task(&item.task);
        Self {
            id: item.id.clone(),
            title,
            status: item.status,
            prio: item.prio,
            due: item.due,
            tag: item.tag.clone(),
        }
    }
}
