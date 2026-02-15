use crate::cli::Cmd;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "todo",
    version,
    about = "A simple todo cli to help you get things done from the comfort of your terminal"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Cmd>,
}
