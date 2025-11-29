use chrono::prelude::*;
use chrono::Local;
use rusqlite::{
    types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef},
    Result,
};
use std::fmt;

use crate::domain::{Datetime, Prio, Status, Tag};

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

impl FromSql for Datetime {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(timestamp) => Ok(Datetime { timestamp }),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Datetime {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.timestamp))
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let today = Local::now().date_naive();
        let yesterday = today.pred_opt().unwrap(); // safe since epoch
        let tomrrow = today.succ_opt().unwrap(); // safe until end of time
        let date = DateTime::from_timestamp(self.timestamp, 0).unwrap();
        let this = date.with_timezone(&Local).date_naive();
        match this {
            _ if *self == Datetime::epoch() => write!(f, ""),
            dt if dt == yesterday => write!(f, "Yesterday"),
            dt if dt == today => write!(f, "Today"),
            dt if dt == tomrrow => write!(f, "Tomorrow"),
            _ => write!(f, "{}", date.naive_local().format("%Y-%m-%d")),
        }
    }
}
