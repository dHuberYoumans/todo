use clap::Subcommand;

use crate::cli::{AddArgs, ClearArgs, CompletionsCmd, GrepArgs, ListArgs, UpdateArgs};
use crate::domain::{Prio, StatusFilter};

#[derive(Subcommand, Debug, Clone)]
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
    Add(AddArgs),
    /// Print the current todo list
    List(ListArgs),
    /// Show metadata of a task
    Show { id: String },
    /// Mark a task as completed
    Close { ids: Vec<String> },
    /// Mark all tasks as completed
    CloseAll {
        #[arg(long, short = 'p', help = "Close all PX tasks")]
        prio: Option<Prio>,
    },
    /// Open a task
    Open { ids: Vec<String> },
    /// Delete a task
    Delete { id: String },
    /// Delete all tasks in the current todo list
    DeleteAll,
    /// Search a pattern inside todos
    Grep(GrepArgs),
    /// Reword a task
    Reword {
        id: String,
        #[arg(long, short = 'm', help = "Task description")]
        task: Option<String>,
    },
    /// Get a random todo among those with prio = RNG
    RND,
    /// Update the fields of an item
    Update(UpdateArgs),
    /// Clear due, prio or the tag column
    Clear(ClearArgs),
    /// Upgrade the CLI
    Upgrade {
        #[arg(long, short = 'v', help = "Version")]
        version: Option<String>,
        #[arg(long, help = "Check latest version")]
        check: bool,
    },
    /// Show user paths
    ShowPaths,
    /// Clean data
    CleanData,
    /// Generates auto-completions
    Completions {
        #[command(subcommand)]
        cmd: CompletionsCmd,
    },
}

impl Default for Cmd {
    fn default() -> Self {
        Cmd::List(ListArgs {
            cmd: None,
            status: Some(StatusFilter::Do),
            prio: None,
            due: None,
            tag: None,
            sort: None,
            arg: None,
        })
    }
}
