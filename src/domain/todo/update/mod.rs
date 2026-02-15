pub mod close_all;
pub mod update_item;
pub mod update_task;

use crate::domain::{Datetime, Prio, Status, Tag};

#[derive(Clone, Debug)]
pub struct UpdateOptions {
    pub due: Option<Datetime>,
    pub prio: Option<Prio>,
    pub status: Option<Status>,
    pub tag: Option<Tag>,
}

#[derive(Clone, Debug)]
pub struct ClearOptions {
    pub due: bool,
    pub prio: bool,
    pub tag: bool,
}
