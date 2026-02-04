use crate::domain::Datetime;

#[derive(Debug)]
pub struct Metadata {
    pub created_at: Datetime,
    pub last_updated: Datetime,
}
