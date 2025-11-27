use clap::Parser;
use std::process;

use todo::domain::todo::Args;
use todo::run::run;

fn main() {
    let args = Args::parse();
    env_logger::init();
    if let Err(err) = run(args) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
