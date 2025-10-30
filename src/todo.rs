use clap::{Parser, Subcommand};
use log;
use std::path::PathBuf;

use crate::queries::schema::{Prio, TodoItem};
use crate::util;

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
    WhoIsThis,
    /// Add a task
    Add {
        #[arg(long, short = 'm', help = "Task description")]
        task: Option<String>,
        #[arg(long, short = 'p', help = "Priority")]
        prio: Option<Prio>,
        #[arg(long, short = 'd', help = "Due date")]
        due: Option<String>,
        #[arg(long, short = 't', help = "Tag")]
        tag: Option<String>,
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
    Close { id: i64 },
    /// Open a task
    Open { id: i64 },
    /// Delete a task
    Delete { id: i64 },
    /// Delete all tasks in the current todo list
    DeleteAll,
    /// Reword a task
    Reword {
        id: i64,
        #[arg(long, short = 'm', help = "Task description")]
        task: Option<String>,
    },
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TodoList {
    pub tasks: Vec<TodoItem>,
    pub db_path: Option<PathBuf>,
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoList {
    pub fn new() -> Self {
        log::debug!("instantiating new 'todo' struct");
        let db_path = util::get_db_path();
        Self {
            tasks: Vec::new(),
            db_path,
        }
    }
}
