use crate::domain::update::ClearOptions;

#[derive(clap::Args, Clone, Debug)]
pub struct ClearArgs {
    pub ids: Vec<String>,
    #[arg(long, help = "Clear the due column")]
    pub due: bool,
    #[arg(long, help = "Clear the prio column")]
    pub prio: bool,
    #[arg(long, help = "Clear the tag column")]
    pub tag: bool,
}

impl From<&ClearArgs> for ClearOptions {
    fn from(args: &ClearArgs) -> Self {
        Self {
            due: args.due,
            prio: args.prio,
            tag: args.tag,
        }
    }
}
