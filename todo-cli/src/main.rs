use clap::Parser;
use std::process;

use todo_core::domain::todo::Cli;
use todo_core::run::run;

fn main() {
    let args = Cli::parse();
    env_logger::init();
    if let Err(err) = run(args) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
