use tabled::Tabled;

use crate::domain::{Datetime, Prio, Status, Tag};
use crate::util::parse_task;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct TodoList {
    pub tasks: Vec<TodoItem>,
}
impl TodoList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
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

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ListFilter {
    None,
    Do,
    Done,
}
