use chrono::Duration;
use std::str::FromStr;
use thiserror::Error;

use chrono::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Datetime {
    pub timestamp: i64,
}

#[derive(Error, Debug)]
pub enum DatetimeParseError {
    #[error("✘ Invalid date format.\nUse either of the following:\n* today\n* tomorrow\n* 3 letter days for the next weekday\n* dd-mm-yyyy for a specific day")]
    InvalidFormat,
}

impl Default for Datetime {
    fn default() -> Self {
        Datetime::now()
    }
}

impl FromStr for Datetime {
    type Err = DatetimeParseError;

    fn from_str(date: &str) -> Result<Self, Self::Err> {
        Datetime::parse(date)
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

    pub fn epoch() -> Datetime {
        let epoch_local = DateTime::<Local>::from(DateTime::UNIX_EPOCH);
        Datetime {
            timestamp: epoch_local.timestamp(),
        }
    }

    pub fn parse(input: &str) -> Result<Datetime, DatetimeParseError> {
        let target = match input.to_lowercase().as_str() {
            day if day.starts_with("mon") => Some(Weekday::Mon),
            day if day.starts_with("tue") => Some(Weekday::Tue),
            day if day.starts_with("wed") => Some(Weekday::Wed),
            day if day.starts_with("thu") => Some(Weekday::Thu),
            day if day.starts_with("fri") => Some(Weekday::Fri),
            day if day.starts_with("sat") => Some(Weekday::Sat),
            day if day.starts_with("sun") => Some(Weekday::Sun),
            "eow" => Some(Weekday::Fri),
            _ => None,
        };
        if let Some(target) = target {
            let today = Local::now();
            let mut date = today.date_naive();
            while date.weekday() != target {
                date += Duration::days(1);
            }
            let naive_dt = date.and_hms_opt(0, 0, 0).unwrap();
            let local_dt = naive_dt.and_local_timezone(Local).unwrap();
            Ok(Datetime {
                timestamp: local_dt.timestamp(),
            })
        } else {
            match input {
                day if ["eod", "today"].contains(&day) => {
                    let today = Local::now().date_naive();
                    let naive_dt = today.and_hms_opt(0, 0, 0).unwrap();
                    let local_dt = naive_dt.and_local_timezone(Local).unwrap();
                    Ok(Datetime {
                        timestamp: local_dt.timestamp(),
                    })
                }
                "tomorrow" => {
                    let today = Local::now().date_naive();
                    let tomorrow = today.succ_opt().unwrap(); // safe until end of time
                    let tomorrow_dt = Local
                        .from_local_datetime(&tomorrow.and_time(NaiveTime::MIN))
                        .unwrap();
                    Ok(Datetime {
                        timestamp: tomorrow_dt.timestamp(),
                    })
                }
                "yesterday" => {
                    let today = Local::now().date_naive();
                    let yesteday = today.pred_opt().unwrap(); // safe until end of time
                    let yesterday_dt = Local
                        .from_local_datetime(&yesteday.and_time(NaiveTime::MIN))
                        .unwrap();
                    Ok(Datetime {
                        timestamp: yesterday_dt.timestamp(),
                    })
                }
                _ => {
                    let date = NaiveDate::parse_from_str(input, "%d-%m-%Y")
                        .map_err(|_| DatetimeParseError::InvalidFormat)?;
                    let naive_dt = date.and_hms_opt(0, 0, 0).unwrap();
                    let local_dt = naive_dt.and_local_timezone(Local).unwrap();
                    Ok(Datetime {
                        timestamp: local_dt.timestamp(),
                    })
                }
            }
        }
    }

    pub fn next_weekday(date: DateTime<Local>, next_weekday: Weekday) -> NaiveDate {
        let today = date.date_naive();
        let weekday = today.weekday();
        let days_until_friday = (next_weekday.num_days_from_monday() as i64
            - weekday.num_days_from_monday() as i64
            + 7)
            % 7;
        (date + Duration::days(days_until_friday)).date_naive()
    }
}
