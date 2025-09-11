use std::{env, fmt, path::PathBuf};
use chrono::prelude::*;
use dirs::home_dir;
use tabled::Tabled;
use rusqlite::{Result, types::{FromSql, ToSql, ValueRef, FromSqlError, FromSqlResult, ToSqlOutput}};
use dotenv::from_filename;

#[derive(Debug, Tabled, PartialEq, PartialOrd)]
pub struct TodoItem{
    pub id: i64,
    pub task: String,
    pub status: Status,
    pub created_at: Datetime,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Status{
    Closed,
    Open
}

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(0) => Ok(Status::Open),
            ValueRef::Integer(1) => Ok(Status::Closed),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Status {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let value = match self {
            Status::Open => 0,
            Status::Closed => 1,
        };
        Ok(ToSqlOutput::from(value))
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Closed => write!(f, "✘"), 
            Status::Open => write!(f, "✔︎"), 
        }
    }
}



#[derive(Debug, PartialEq, PartialOrd)]
pub struct Datetime{
    pub timestamp: DateTime<Local>,
}

impl Datetime {
    pub fn new() -> Self {
        Self { 
            timestamp: Local::now(),
        }
    }
}


impl FromSql for Datetime {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(timestamp) => {
                let utc_time = DateTime::from_timestamp(timestamp, 0).ok_or(FromSqlError::InvalidType)?;
                Ok(Datetime { timestamp:  DateTime::with_timezone(&utc_time,&Local)})
            }
            _ => Err(FromSqlError::InvalidType)
        }
    }}

impl ToSql for Datetime {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.timestamp.timestamp()))
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}", self.timestamp.format("%Y-%m-%d")) 
    }
}

pub fn get_todo_list_path() -> Option<PathBuf> {
    let parent = home_dir()?.join(".todo");
    let path = parent
        .clone()
        .join(".env");
    from_filename(&path).ok();
    let db = env::var("TODO_DB").ok()?;
    Some(PathBuf::from(&path.parent()?).join(db))
}

pub fn get_todo_dir() -> Option<PathBuf> {
    Some(home_dir()?.join(".todo"))
}

pub fn get_env_path() -> Option<PathBuf> {
    Some(home_dir()?.join(".todo/.env"))
}
