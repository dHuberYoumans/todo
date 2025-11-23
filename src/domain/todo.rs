use clap::{Parser, Subcommand};
use log;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::PathBuf;
use tabled::Tabled;

use crate::domain::{Datetime, Prio, Status, Tag};
use crate::paths::UserPaths;

#[derive(Parser, Debug)]
#[command(
    name = "todo",
    version,
    about = "A simple todo cli to help you get things done from the comfort of your terminal"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Cmd>,
    #[arg(long, short = 'v', help = "verbose")]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Initialize the cli in CWD  
    Init,
    /// Open config
    Config,
    /// Create a new todo list
    NewList {
        name: String,
        #[arg(long, short = 'c', help = "Directly load new list")]
        checkout: bool,
    },
    /// Delete a todo list
    DeleteList { name: String },
    /// Load a todo list
    Load { name: String },
    /// Print the name of the todo list in use to stdout
    Whoami,
    /// Add a task
    Add {
        #[arg(long, short = 'm', help = "Task description")]
        task: Option<String>,
        #[arg(long, short = 'p', help = "Priority")]
        prio: Option<Prio>,
        #[arg(long, short = 'd', help = "Due date")]
        due: Option<Datetime>,
        #[arg(long, short = 't', help = "Tag")]
        tag: Option<Tag>,
    },
    /// Print the current todo list
    List {
        #[arg(long, short = 'a', help = "Show all tasks")]
        all: bool,
        #[arg(long, help = "Show all completed tasks")]
        done: bool,
        #[arg(long, short = 's', help = "Sort tasks")]
        sort: Option<String>,
        #[arg(long, help = "Show collection")]
        collection: bool,
        #[arg(long, help = "Display available tags")]
        tags: bool,
        arg: Option<String>,
    },
    /// Mark a task as completed
    Close { id: String },
    /// Open a task
    Open { id: String },
    /// Delete a task
    Delete { id: String },
    /// Delete all tasks in the current todo list
    DeleteAll,
    /// Reword a task
    Reword {
        id: String,
        #[arg(long, short = 'm', help = "Task description")]
        task: Option<String>,
    },
    /// Update the fields of an item
    Update {
        id: String,
        #[arg(long, short = 'd', help = "Update the due date")]
        due: Option<Datetime>,
        #[arg(long, short = 'p', help = "Update the priority")]
        prio: Option<Prio>,
        #[arg(long, short = 's', help = "Update the status")]
        status: Option<Status>,
        #[arg(long, short = 't', help = "Update the tag")]
        tag: Option<Tag>,
    },
    /// Show user paths
    ShowPaths,
    /// Clean data
    CleanData,
}

#[derive(Debug, PartialEq, PartialOrd)]
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

#[derive(Debug, Tabled, PartialEq, PartialOrd)]
pub struct TodoItem {
    pub id: String,
    pub task: String,
    pub status: Status,
    pub prio: Prio,
    pub due: Datetime,
    pub tag: Tag,
}

impl Hash for TodoItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.task.hash(state);
        self.status.hash(state);
        self.prio.hash(state);
        self.due.hash(state);
        self.tag.hash(state);
    }
}

impl TodoItem {
    pub fn hash_id(&mut self) {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hashed_id = hasher.finish() as i64;
        let hex_id = format!("{:x}", hashed_id);
        self.id = hex_id;
    }
}
