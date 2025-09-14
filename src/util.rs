use std::{
    fs,
    env,
    fmt,
    path::PathBuf,
    error::Error,
    process,
    io::Read
};
use glob;
use chrono::{prelude::*};
use chrono::{
    Datelike,
    Duration,
    Local,
    Weekday,
    NaiveDate
};
use dirs::home_dir;
use tabled::Tabled;
use rusqlite::{
    Result,
    types::{
        FromSql,
        ToSql,
        ValueRef,
        FromSqlError,
        FromSqlResult,
        ToSqlOutput
    }
};
use dotenv::from_filename;

const TMP_FILE: &str = "./EDIT_TASK";

#[derive(Debug, Tabled, PartialEq, PartialOrd)]
pub struct TodoItem{
    pub id: i64,
    pub task: String,
    pub status: Status,
    pub prio: Prio,
    pub due: Datetime,
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

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Prio{
    P1,
    P2,
    P3,
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
            Prio::Empty => 0
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

pub fn parse_date(input: &str) -> Result<Datetime, Box<dyn Error>> {
    let target = match input.to_lowercase().as_str() {
        "mon" => Some(Weekday::Mon),
        "tue" => Some(Weekday::Tue),
        "wed" => Some(Weekday::Wed),
        "thu" => Some(Weekday::Thu),
        "fri" => Some(Weekday::Fri),
        "sat" => Some(Weekday::Sat),
        "sun" => Some(Weekday::Sun),
        _ => None,
    };

    if let Some(target) = target {
        let today = Local::now();
        let mut date = today.date_naive();
        while date.weekday() != target {
            date += Duration::days(1);
        }
        let naive_dt = date.and_hms_opt(0, 0, 0).unwrap();
        let local_dt = Local.from_local_datetime(&naive_dt).unwrap();
        Ok(Datetime { timestamp: local_dt })
    } 
    else {
        match input {
            "today" => Ok(Datetime::new()),
            "tomorrow" => {
                let today = Local::now().date_naive();
                let tomorrow = today.succ_opt().unwrap(); // safe until end of time
                let tomorrow_dt = Local.from_local_datetime(&tomorrow.and_time(NaiveTime::MIN)).unwrap();
                Ok( Datetime { timestamp:  tomorrow_dt})
            },
            _ => {
                let date = NaiveDate::parse_from_str(input, "%d-%m-%Y")
            .map_err(|_| "✘ Invalid date format.\nUse either of the following:\n* today\n* tomorrow\n* 3 letter days for the next weekday\n* dd-mm-yyyy for a specific day")?;
        let naive_dt = date.and_hms_opt(0,0,0).unwrap();
        let local_dt = Local.from_local_datetime(&naive_dt).single().unwrap();
        Ok(Datetime { timestamp: local_dt})
            }
        }
    }
}

pub fn epoch() -> Datetime {
    let epoch_local = DateTime::<Local>::from(DateTime::UNIX_EPOCH);
    Datetime { timestamp: epoch_local }
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

pub fn edit_in_editor(old_text: Option<String>) -> String {
    let editor = env::var("EDITOR").unwrap_or(String::from("vi"));
    let path = PathBuf::from(TMP_FILE);
    fs::File::create(&path)
        .expect(format!("✘ Could not open file {}", TMP_FILE).as_str());
    if let Some(text) = old_text {
        fs::write(&path,text).expect(format!("✘ Could not write to file {}", TMP_FILE).as_str());
    };
    process::Command::new(editor)
        .arg(&path)
        .status()
        .expect("✘ Couldn't open your editor");
    let mut task = String::new(); 
    fs::File::open(&path)
        .expect(format!("✘ Could not open file {}", TMP_FILE).as_str())
        .read_to_string(&mut task)
        .expect("✘ Couldn't parse task");
    cleanup_tmp_files().expect("✘ Error during cleanup");
    return task;
}

fn cleanup_tmp_files() -> Result<(), Box<dyn Error>> {
    let pattern = format!("{TMP_FILE}*");
    for file in glob::glob(&pattern)? {
        match file {
            Ok(path) => std::fs::remove_file(path)?,
            Err(e) => eprintln!("✘ Could not find file: {e}"),
        }
    }
    Ok(())
}
