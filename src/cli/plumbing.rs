use crate::cli::{Cmd, CompletionsCmd};
use anyhow::Result;

#[derive(Debug)]
pub enum Plumbing {
    ShowPaths,
    CleanData,
    Init,
    Completions(CompletionsCmd),
}

impl TryFrom<&Cmd> for Plumbing {
    type Error = ();

    fn try_from(cmd: &Cmd) -> Result<Self, Self::Error> {
        match cmd {
            Cmd::Init => Ok(Plumbing::Init),
            Cmd::CleanData => Ok(Plumbing::CleanData),
            Cmd::ShowPaths => Ok(Plumbing::ShowPaths),
            Cmd::Completions { cmd } => Ok(Plumbing::Completions(cmd.clone())),
            _ => Err(()),
        }
    }
}
