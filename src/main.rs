use std::process;
use clap::Parser;

use todo::run::run;
use todo::todo::Args;

fn main() {
    let args = Args::parse(); 
    if let Err(err) = run(args){
        eprintln!("{}", err);
        process::exit(1);
    }
}
