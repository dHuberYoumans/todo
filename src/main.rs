use clap::Parser;
use std::process;

use todo::application::app::App;
use todo::application::run::run;

fn main() {
    let app = App::parse();
    env_logger::init();
    if let Err(err) = run(app) {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}
