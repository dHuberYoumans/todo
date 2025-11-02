use chrono::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub struct Datetime {
    pub timestamp: DateTime<Local>,
}

impl Default for Datetime {
    fn default() -> Self {
        Datetime::now()
    }
}

impl Datetime {
    pub fn new(timestamp: Option<i64>) -> Self {
        let ts = match timestamp {
            Some(dt) => Local
                .timestamp_opt(dt, 0)
                .single()
                .unwrap_or_else(Local::now),
            None => Local::now(),
        };
        Self { timestamp: ts }
    }

    pub fn now() -> Self {
        Self::new(None)
    }
}
