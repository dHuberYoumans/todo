use chrono::prelude::*;
use chrono::Local;
use clap::ValueEnum;
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result,
};
use std::fmt;
use tabled::Tabled;

#[derive(Debug, Tabled, PartialEq, PartialOrd)]
pub struct TodoItem {
    pub id: i64,
    pub task: String,
    pub status: Status,
    pub prio: Prio,
    pub due: Datetime,
    pub tag: Tag,
}

#[derive(Debug, PartialEq, PartialOrd, ValueEnum, Clone)]
pub enum Status {
    Closed,
    Open,
}

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(1) => Ok(Status::Open),
            ValueRef::Integer(0) => Ok(Status::Closed),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Status {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let value = match self {
            Status::Open => 1,
            Status::Closed => 0,
        };
        Ok(ToSqlOutput::from(value))
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Open => write!(f, "✘"),
            Status::Closed => write!(f, "✔"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Tag(pub String);

impl ToSql for Tag {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.0.clone()))
    }
}

impl FromSql for Tag {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(bytes) => {
                let sql_str = std::str::from_utf8(bytes)
                    .map_err(|_| FromSqlError::Other("Invalid UTF-8".into()))?;
                let stripped = sql_str.strip_prefix("#").unwrap_or(sql_str);
                Ok(Tag(stripped.to_string()))
            }
            _ => Ok(Tag(String::new())),
        }
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "")
        } else {
            write!(f, "#{}", self.0)
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Default, ValueEnum)]
pub enum Prio {
    P1,
    P2,
    P3,
    #[default]
    Empty,
}

impl FromSql for Prio {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(1) => Ok(Prio::P1),
            ValueRef::Integer(2) => Ok(Prio::P2),
            ValueRef::Integer(3) => Ok(Prio::P3),
            ValueRef::Integer(0) => Ok(Prio::Empty),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Prio {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let value = match self {
            Prio::P1 => 1,
            Prio::P2 => 2,
            Prio::P3 => 3,
            Prio::Empty => 0,
        };
        Ok(ToSqlOutput::from(value))
    }
}

impl fmt::Display for Prio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prio::P1 => write!(f, "P1"),
            Prio::P2 => write!(f, "P2"),
            Prio::P3 => write!(f, "P3"),
            Prio::Empty => write!(f, ""),
        }
    }
}

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

impl FromSql for Datetime {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(timestamp) => {
                let utc_time =
                    DateTime::from_timestamp(timestamp, 0).ok_or(FromSqlError::InvalidType)?;
                Ok(Datetime {
                    timestamp: DateTime::with_timezone(&utc_time, &Local),
                })
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Datetime {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.timestamp.timestamp()))
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let today = Local::now().date_naive();
        let yesterday = today.pred_opt().unwrap(); // safe since epoch
        let tomrrow = today.succ_opt().unwrap(); // safe until end of time
        let this = self.timestamp.date_naive();
        match this {
            _ if *self == epoch() => write!(f, ""),
            dt if dt == yesterday => write!(f, "Yesterday"),
            dt if dt == today => write!(f, "Today"),
            dt if dt == tomrrow => write!(f, "Tomorrow"),
            _ => write!(f, "{}", self.timestamp.format("%Y-%m-%d")),
        }
    }
}

pub fn epoch() -> Datetime {
    let epoch_local = DateTime::<Local>::from(DateTime::UNIX_EPOCH);
    Datetime {
        timestamp: epoch_local,
    }
}
