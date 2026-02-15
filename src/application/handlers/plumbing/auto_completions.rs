use anyhow::{Context, Result};
use std::fs;

use clap::CommandFactory;
use clap_complete::{generate, Shell};

use crate::cli::app::Cli;
use crate::infrastructure::config;
use crate::infrastructure::paths::UserPaths;

pub fn generate_completions(shell: Shell) -> Result<()> {
    let mut cli_builder = Cli::command();
    generate(shell, &mut cli_builder, "todo", &mut std::io::stdout());
    Ok(())
}

pub fn install_completions(shell: Shell) -> Result<()> {
    let user_paths = UserPaths::new();
    match shell {
        Shell::Zsh => install(
            Shell::Zsh,
            user_paths.home.join(".zsh").join("completions"),
            "_todo",
        ),
        Shell::Bash => install(
            Shell::Bash,
            user_paths
                .home
                .join(".local/share/bash-completion/completions"),
            "todo",
        ),
        Shell::Fish => install(
            Shell::Fish,
            config::get_config(&user_paths)?.join("fish/completions"),
            "todo.fish",
        ),
        _ => anyhow::bail!("{shell} shell not supported"),
    }
}

fn install(shell: Shell, dir: std::path::PathBuf, filename: &str) -> Result<()> {
    println!("▶ Installing auto completions...");
    fs::create_dir_all(&dir).context("✘ Couldn't create directory")?;
    let path = dir.join(filename);
    let mut file = fs::File::create(&path).context("✘ Couldn't create file")?;
    let mut cli_builder = Cli::command();
    generate(shell, &mut cli_builder, "todo", &mut file);
    println!("✔ Installed {} completions to {}", shell, path.display());
    println!("ℹ Restart your terminal to activate");
    Ok(())
}
