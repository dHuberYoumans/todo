use crate::domain::ListFilter;
use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum ListSubCmd {
    /// List collection (of todo lists)
    Collection,
    /// List all tags present in the todo list
    Tags,
}

#[derive(clap::Args, Clone, Debug)]
pub struct ListArgs {
    #[command(subcommand)]
    pub cmd: Option<ListSubCmd>,
    #[arg(long, value_enum, help = "Filter tasks")]
    pub filter: Option<ListFilter>,
    #[arg(long, short = 's', help = "Sort tasks")]
    pub sort: Option<String>,
    /// Optional positional argument like @today or #work
    pub arg: Option<String>,
}
