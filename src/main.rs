use clap::Parser;
use std::process;

use todo::app::App;
use todo::run::run;

fn main() {
    let app = App::parse();
    env_logger::init();
    if let Err(err) = run(app) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
