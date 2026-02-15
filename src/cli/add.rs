use crate::application::config::Config;
use crate::domain::{add_item::AddOptions, Datetime, Prio, Tag};
use anyhow::Result;

#[derive(clap::Args, Clone, Debug)]
pub struct AddArgs {
    #[arg(long, short = 'm', help = "Task description")]
    pub task: Option<String>,
    #[arg(long, short = 'p', help = "Priority")]
    pub prio: Option<Prio>,
    #[arg(long, short = 'd', help = "Due date")]
    pub due: Option<String>, // Use String instead of Datetime for config-dependent parsing
    #[arg(long, short = 't', help = "Tag")]
    pub tag: Option<Tag>,
}

impl AddArgs {
    pub fn into_options(self, config: &Config) -> Result<AddOptions> {
        let due = match self.due {
            Some(s) => Some(Datetime::parse(
                &s,
                config.style.due_date_input_format.clone(),
            )?),
            None => None,
        };
        Ok(AddOptions {
            task: self.task,
            prio: self.prio,
            due,
            tag: self.tag,
        })
    }
}
