use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh},
};
use std::{env, fs, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let completions_dir = out_dir.join("completions");
    fs::create_dir_all(&completions_dir).unwrap();
    let mut cmd = todo_core::domain::build_cli();
    let bin_name = "todo";
    generate_to(Bash, &mut cmd, bin_name, &completions_dir).unwrap();
    generate_to(Zsh, &mut cmd, bin_name, &completions_dir).unwrap();
    generate_to(Fish, &mut cmd, bin_name, &completions_dir).unwrap();
    println!(
        "cargo:warning=Completions generated in {:?}",
        completions_dir
    );
    // rerun if the CLI definition changes
    println!("cargo:rerun-if-changed=../todo-core/src/domain/todo.rs");
    println!("cargo:rerun-if-changed=../todo-core/src/domain/mod.rs");
}
