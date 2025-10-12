use clap::Parser;
use std::process;

use todo::paths::UserPaths;
use todo::run::run;
use todo::todo::Args;

fn main() {
    let env_path = UserPaths::new().home.join(".todo").join(".env");
    if dotenv::from_filename(&env_path).is_err() {
        eprintln!("✘ No .env file found at {env_path:?}");
    }
    let args = Args::parse();
    if args.verbose {
        std::env::set_var("RUST_LOG", "info");
        println!(
            "RUST_LOG={}",
            std::env::var("RUST_LOG").expect("RUST_LOG not set")
        );
    };
    env_logger::init();
    if let Err(err) = run(args) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
