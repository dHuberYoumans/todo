use crate::commands::Cmd;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "todo",
    version,
    about = "A simple todo cli to help you get things done from the comfort of your terminal"
)]
pub struct App {
    #[command(subcommand)]
    pub command: Option<Cmd>,
}
