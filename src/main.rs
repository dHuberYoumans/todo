use clap::Parser;
use std::process;

use todo::application::run::run;
use todo::cli::app::Cli;

fn main() {
    let app = Cli::parse();
    env_logger::init();
    if let Err(err) = run(app) {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}
