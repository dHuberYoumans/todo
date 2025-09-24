use std::process;
use clap::Parser;
use env_logger;

use todo::run::run;
use todo::todo::Args;

fn main() {
    let args = Args::parse(); 
    if args.verbose {
        std::env::set_var("RUST_LOG", "info");
        println!("RUST_LOG={}", std::env::var("RUST_LOG").expect("RUST_LOG not set"));
    };
    env_logger::init();
    if let Err(err) = run(args){
        eprintln!("{}", err);
        process::exit(1);
    }
}
