use crate::domain::grep::GrepOptions;

#[derive(clap::Args, Clone, Debug)]
pub struct GrepArgs {
    pub pattern: String,
    #[arg(long, short = 'i', help = "Search case-insensitively")]
    pub ignore: bool,
}

impl From<&GrepArgs> for GrepOptions {
    fn from(args: &GrepArgs) -> Self {
        Self {
            case_insensitive: args.ignore,
        }
    }
}
