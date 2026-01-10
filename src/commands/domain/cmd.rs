use clap::Subcommand;

use crate::domain::{Datetime, Prio, Status, Tag};

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
    List(ListArgs),
    /// Show metadata of a task
    Show { id: String },
    /// Mark a task as completed
    Close { ids: Vec<String> },
    /// Open a task
    Open { ids: Vec<String> },
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
        ids: Vec<String>,
        #[arg(long, short = 'd', help = "Update the due date")]
        due: Option<Datetime>,
        #[arg(long, short = 'p', help = "Update the priority")]
        prio: Option<Prio>,
        #[arg(long, short = 's', help = "Update the status")]
        status: Option<Status>,
        #[arg(long, short = 't', help = "Update the tag")]
        tag: Option<Tag>,
    },
    /// Clear due, prio or the tag column
    Clear {
        ids: Vec<String>,
        #[arg(long, help = "Clear the due column")]
        due: bool,
        #[arg(long, help = "Clear the prio column")]
        prio: bool,
        #[arg(long, help = "Clear the tag column")]
        tag: bool,
    },
    /// Upgrade the CLI
    Upgrade {
        #[arg(long, short = 'v', help = "Version")]
        version: Option<String>,
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

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ListFilter {
    None,
    Do,
    Done,
}

#[derive(clap::Args, Clone, Debug)]
pub struct ListArgs {
    #[arg(long, value_enum, help = "Filter tasks")]
    pub filter: Option<ListFilter>,
    #[arg(long, short = 's', help = "Sort tasks")]
    pub sort: Option<String>,
    #[arg(long, help = "Show collection")]
    pub collection: bool,
    #[arg(long, help = "Display available tags")]
    pub tags: bool,
    pub arg: Option<String>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum CompletionsCmd {
    /// Print completions to stdout
    Generate {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    /// Install completions for the given shell
    Install {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

impl Default for Cmd {
    fn default() -> Self {
        Cmd::List(ListArgs {
            filter: Some(ListFilter::Do),
            sort: None,
            collection: false,
            tags: false,
            arg: None,
        })
    }
}

#[derive(Debug)]
pub enum Plumbing {
    ShowPaths,
    CleanData,
    Init,
    Completions(CompletionsCmd),
}

impl TryFrom<&Cmd> for Plumbing {
    type Error = ();

    fn try_from(cmd: &Cmd) -> Result<Self, Self::Error> {
        match cmd {
            Cmd::Init => Ok(Plumbing::Init),
            Cmd::CleanData => Ok(Plumbing::CleanData),
            Cmd::ShowPaths => Ok(Plumbing::ShowPaths),
            Cmd::Completions { cmd } => Ok(Plumbing::Completions(cmd.clone())),
            _ => Err(()),
        }
    }
}
