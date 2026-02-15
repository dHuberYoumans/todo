use crate::domain::{update::UpdateOptions, Datetime, Prio, Status, Tag};

#[derive(clap::Args, Clone, Debug)]
pub struct UpdateArgs {
    pub ids: Vec<String>,
    #[arg(long, short = 'd', help = "Update the due date")]
    pub due: Option<Datetime>,
    #[arg(long, short = 'p', help = "Update the priority")]
    pub prio: Option<Prio>,
    #[arg(long, short = 's', help = "Update the status")]
    pub status: Option<Status>,
    #[arg(long, short = 't', help = "Update the tag")]
    pub tag: Option<Tag>,
}

impl From<&UpdateArgs> for UpdateOptions {
    fn from(args: &UpdateArgs) -> Self {
        Self {
            due: args.due,
            prio: args.prio,
            status: args.status,
            tag: args.tag.clone(),
        }
    }
}
