use chrono::prelude::*;

#[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Datetime {
    pub timestamp: i64,
}

impl Default for Datetime {
    fn default() -> Self {
        Datetime::now()
    }
}

impl Datetime {
    pub fn new(timestamp: Option<i64>) -> Self {
        let ts = if let Some(timestamp) = timestamp {
            timestamp
        } else {
            Local::now().timestamp()
        };
        Self { timestamp: ts }
    }

    pub fn now() -> Self {
        Self::new(None)
    }

    pub fn from(date: &str) -> Self {
        let date = NaiveDate::parse_from_str(date, "%d-%m-%Y")
            .expect("Invalid date. Please enter date in dd-mm-yyyy format");
        let local_dt = Local
            .from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
            .unwrap();
        let timestamp = local_dt.timestamp();
        Self { timestamp }
    }
}
