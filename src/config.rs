use std::fmt;

use crate::todo::Cmd;
use rusqlite::{ Result, types::{FromSql, ToSql, ValueRef, FromSqlError, FromSqlResult, ToSqlOutput}};

pub struct Config{
    pub command: Cmd,
    pub args: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Config { command: Cmd::Idle, args: None}
    }
}

impl Config {
    pub fn new(cl_args: &[String]) -> Result<Config, String>{
        if cl_args.len() == 1 { 
            return Ok(Config {command: Cmd::List, args: None});
        }
        let command: String = cl_args[1].clone();
        let mut config = Config::default();
                match command.as_str() {
            "init" => {
                config.command = Cmd::Init;
                config.args = None;
            },
            "new-list" => {
                config.command = Cmd::NewList;
                config.args = Some(vec![cl_args[2].clone()]);
            },
            "delete-list" => {
                config.command = Cmd::DeleteList;
                config.args = Some(vec![cl_args[2].clone()]);
            },
            "load" => {
                config.command = Cmd::Load;
                config.args = Some(vec![cl_args[2].clone()]);
            }
            "add" => { 
                config.command = Cmd::Add;
                config.args = Some(vec![cl_args[2].clone()]);
            },
            "list" => {
                config.command = Cmd::List;
                config.args = cl_args.get(2).map(|arg| vec![arg.clone()]);
            },
            "close" => {
                config.command = Cmd::Close;
                config.args = Some(vec![cl_args[2].clone()]);
            }, 
            "open" => {
                config.command = Cmd::Open;
                config.args = Some(vec![cl_args[2].clone()]);
            },
            "delete" => {
                config.command = Cmd::Delete;
                config.args = Some(vec![cl_args[2].clone()]);
            },
            "delete-all" => {
                config.command = Cmd::DeleteAll;
                config.args = None;
            },
            "reword" => {
                config.command = Cmd::Reword;
                config.args = Some(vec![cl_args[2].clone(), cl_args[3].clone()]);
            },
            "help" => {
                config.command = Cmd::Help;
                config.args = None;
            },
            _ => { return Err(String::from("Invalid command")); }
        }
        Ok(config)
    }
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

