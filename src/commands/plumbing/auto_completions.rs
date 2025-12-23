use anyhow::Result;
use std::fs;

use clap::CommandFactory;
use clap_complete::{generate, Shell};

use crate::app::App;
use crate::domain::TodoList;
use crate::paths::UserPaths;

impl TodoList {
    pub fn generate_completions(shell: Shell) -> Result<()> {
        let mut cli_builder = App::command();
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
                user_paths.get_config()?.join("fish/completions"),
                "todo.fish",
            ),
            _ => anyhow::bail!("Shell not supported yet"),
        }
    }
}

fn install(shell: Shell, dir: std::path::PathBuf, filename: &str) -> Result<()> {
    fs::create_dir_all(&dir)?;
    let path = dir.join(filename);
    let mut file = fs::File::create(&path)?;
    let mut cli_builder = App::command();
    generate(shell, &mut cli_builder, "todo", &mut file);
    println!("✔ Installed {} completions to {}", shell, path.display());
    println!("ℹ Restart your terminal to activate");
    Ok(())
}
