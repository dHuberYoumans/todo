use clap::Subcommand;

#[derive(Subcommand, Debug, Clone)]
pub enum CompletionsCmd {
    /// Print completions to stdout
    Generate {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    /// Install completions for the given shell
    Install {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}
