use clap::Parser;
use std::process;

use todo::application::config;
use todo::application::run::run;
use todo::cli::app::Cli;
use todo::cli::expand_alias;

fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    let config = match config::load_config() {
        Ok(config) => config,
        Err(err_config) => {
            eprintln!("{:?}", err_config);
            process::exit(1);
        }
    };
    let expanded = expand_alias(args, &config);
    let cli = Cli::parse_from(expanded);
    if let Err(err) = run(cli, &config) {
        eprintln!("{:?}", err);
        process::exit(1);
    }
}
