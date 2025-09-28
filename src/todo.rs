use clap::{Parser, Subcommand};
use log;
use rusqlite::{params, Connection, OptionalExtension, Result, ToSql};
use std::cmp::Reverse;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tabled::{
    settings::{object::Columns, Modify, Style, Width},
    Table,
};

use crate::paths::UserPaths;
use crate::queries;
use crate::util::{self, connect_to_db, epoch, Datetime, Prio, Status, Tag, TodoItem};

#[derive(Parser, Debug)]
#[command(
    name = "todo",
    version,
    about = "A simple todo cli to help you get things done from the comfort of your terminal"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Cmd>,
    #[arg(long, short = 'v', help = "verbose")]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Initialize the cli in CWD  
    Init,
    /// Open config
    Config,
    /// Create a new todo list
    NewList {
        name: String,
        #[arg(long, short = 'c', help = "Directly load new list")]
        checkout: bool,
    },
    /// Delete a todo list
    DeleteList { name: String },
    /// Load a todo list
    Load { name: String },
    /// Print the name of the todo list in use to stdout
    WhoIsThis,
    /// Add a task
    Add {
        #[arg(long, short = 'm', help = "Task description")]
        task: Option<String>,
        #[arg(long, short = 'p', help = "Priority")]
        prio: Option<i64>,
        #[arg(long, short = 'd', help = "Due date")]
        due: Option<String>,
        #[arg(long, short = 't', help = "Tag")]
        tag: Option<String>,
    },
    /// Print the current todo list
    List {
        #[arg(long, short = 'a', help = "Show all tasks")]
        all: bool,
        #[arg(long, help = "Show all completed tasks")]
        done: bool,
        #[arg(long, short = 's', help = "Sort tasks")]
        sort: Option<String>,
        #[arg(long, help = "Show collection")]
        collection: bool,
    },
    /// Mark a task as completed
    Close { id: i64 },
    /// Open a task
    Open { id: i64 },
    /// Delete a task
    Delete { id: i64 },
    /// Delete all tasks in the current todo list
    DeleteAll,
    /// Reword a task
    Reword {
        id: i64,
        #[arg(long, short = 'm', help = "Task description")]
        task: Option<String>,
    },
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TodoList {
    pub tasks: Vec<TodoItem>,
    pub db_path: Option<PathBuf>,
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoList {
    pub fn new() -> Self {
        log::debug!("instantiating new 'todo' struct");
        let db_path = util::get_db_path();
        Self {
            tasks: Vec::new(),
            db_path,
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        println!("⧖ Initializing..");
        let user_paths = UserPaths::new();
        let home = if let Ok(path) = std::env::var("HOME") {
            // hijack env for testing
            PathBuf::from(path)
        } else {
            user_paths.home
        };
        log::debug!("$HOME={:?}", &home);
        let mut file_path = home.to_path_buf();
        file_path.push(".todo/.env");
        if file_path.exists() {
            println!("✔ Environmental setup found");
            return Ok(());
        }
        println!("⧖ Setting up database..");
        fs::create_dir_all(file_path.parent().unwrap())?;
        let mut env = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;
        if let Some(config) = user_paths.config {
            log::debug!("$CONFIG={:?}", &config);
            writeln!(env, "CONFIG={}", config.to_string_lossy())?
        } else {
            log::debug!("CONFIG not found. Setting CONFIG=");
            writeln!(env, "CONFIG=")?
        }
        writeln!(env, "CURRENT=todo")?;
        writeln!(env, "PREVIOUS=todo")?;
        self.db_path = util::get_db_path();
        log::info!("creating database at {}", util::log_opt_path(&self.db_path));
        let conn = if let Some(path) = &self.db_path {
            Connection::open(path)?
        } else {
            return Err("✘ Something went wrong setting up the the database"
                .to_string()
                .into());
        };
        log::info!("creating new collection");
        conn.execute(&queries::create_collection(), [])?;
        log::info!("creating new table");
        self.new_list(String::from("todo"), false)?;
        println!("✔ All done");
        Ok(())
    }

    pub fn config(self) -> Result<(), Box<dyn Error>> {
        let path = std::env::var("CONFIG")?;
        log::info!("read config at {path}");
        let config = fs::read_to_string(&path).ok();
        let new_config = util::edit_in_editor(config);
        log::info!("write new config");
        fs::write(path, new_config)?;
        println!("✔ Config written");
        Ok(())
    }

    pub fn list(&mut self, flags: (Option<String>, Option<String>)) -> Result<(), Box<dyn Error>> {
        let conn = connect_to_db(&self.db_path)?;
        let current_list = std::env::var("CURRENT")?;
        let current_list_id = util::fetch_active_list_id(&self.db_path)?;
        log::debug!(
            "found current list '{}' with ID={}",
            &current_list,
            &current_list_id
        );
        let opt = flags.0.as_deref().unwrap_or("None");
        log::debug!("using option '{opt}'");
        let query = match opt {
            "--all" => format!("SELECT * FROM {current_list} WHERE list_id = ?"),
            "--done" => format!("SELECT * FROM {current_list} WHERE status=0 AND list_id = ?"),
            _ => format!("SELECT * FROM {current_list} WHERE status=1 AND list_id = ?"),
        };
        log::debug!(
            "executing query `{}` \n with params [{}]",
            &query,
            &current_list_id
        );
        let mut stmt = conn.prepare(&query)?;
        let tasks_iter = stmt.query_map(params![current_list_id], |row| {
            Ok(TodoItem {
                id: row.get::<_, i64>("id")?,
                task: row.get::<_, String>("task")?,
                status: row.get::<_, Status>("status")?,
                prio: row.get::<_, Prio>("prio")?,
                due: row.get::<_, Datetime>("due")?,
                tag: row.get::<_, Tag>("tag")?,
            })
        })?;
        for task_result in tasks_iter {
            let task = task_result?;
            self.tasks.push(task);
        }
        let sort_key = flags.1.as_deref().unwrap_or("id");
        log::debug!("using sort key {sort_key}");
        match sort_key {
            "id" => self.tasks.sort_by_key(|entry| Reverse(entry.id)),
            "prio" => self.tasks.sort_by_key(|entry| entry.prio.clone()),
            "tag" => self.tasks.sort_by_key(|entry| entry.tag.clone()),
            "due" => self.tasks.sort_by_key(|entry| {
                let key = entry.due.clone();
                (key == epoch(), key)
            }),
            _ => self.tasks.sort_by_key(|entry| Reverse(entry.id)),
        };
        let mut table = Table::new(&self.tasks);
        table
            .with(Modify::new(Columns::single(0)).with(Width::increase(5))) // id
            .with(Modify::new(Columns::single(1)).with(Width::wrap(60))) // task
            .with(Modify::new(Columns::single(2)).with(Width::increase(3))) // status
            .with(Modify::new(Columns::single(3)).with(Width::increase(3))) // prio
            .with(Modify::new(Columns::single(4)).with(Width::increase(3))) // due
            .with(Modify::new(Columns::single(5)).with(Width::wrap(12))) // tag
            .with(Style::modern_rounded());
        println!("{}", table);
        Ok(())
    }

    pub fn new_list(&mut self, list: String, checkout: bool) -> Result<(), Box<dyn Error>> {
        println!("⧖ Creating new_list..");
        let conn = connect_to_db(&self.db_path)?;
        log::debug!("executing query `{}`", &queries::create_list(&list));
        conn.execute(&queries::create_list(&list), [])?;
        println!("✔ Created new list '{list}'");
        log::debug!("executing query `{}`", &queries::add_to_collection(&list));
        conn.execute(&queries::add_to_collection(&list), [])?;
        println!("✔ Added '{list}' to collection");
        if checkout {
            log::info!("checking out list '{list}'");
            self.load(list.clone())?;
            println!("✔ Now using '{list}'");
        };
        Ok(())
    }

    pub fn delete_list(self, list: String) -> Result<(), Box<dyn Error>> {
        let conn = connect_to_db(&self.db_path)?;
        let dotenv = util::dotenv()?;
        let content = fs::read_to_string(&dotenv)?;
        log::debug!("reading env {:?}", dotenv);
        let mut new_content = String::new();
        for line in content.lines() {
            if line.starts_with("CURRENT=") {
                let current_list = line.split('=').next_back().unwrap_or("");
                if list == current_list {
                    return Err(
                        format!("✘ can't delete the list '{list}' since currently in use").into(),
                    );
                };
                new_content.push_str(&format!("{line}\n"));
            } else if line.starts_with("PREVIOUS=") {
                let current_list = line.split('=').next_back().unwrap_or("");
                if list == current_list {
                    new_content.push_str("PREVIOUS=\n");
                } else {
                    new_content.push_str(&format!("{line}\n"));
                };
            } else {
                new_content.push_str(&format!("{line}\n"));
            }
        }
        log::debug!("executing query `{}`", &queries::delete_list(&list));
        conn.execute_batch(&queries::delete_list(&list))?;
        println!("✔ List '{list}' removed");
        log::debug!("writing dotenv `{new_content}`");
        let mut file = fs::File::create(dotenv)?;
        file.write_all(new_content.as_bytes())?;
        Ok(())
    }

    pub fn list_collection(&self) -> Result<(), Box<dyn Error>> {
        let conn = connect_to_db(&self.db_path)?;
        let mut stmt = conn.prepare(&queries::fetch_collection())?;
        let collection_iter = stmt.query_map([], |row| {
            let list = row.get::<_, String>("name")?;
            Ok(list)
        })?;
        println!("Your collection\n===============");
        for list in collection_iter.flatten() {
            println!("• {list}");
        }
        Ok(())
    }

    pub fn load(&mut self, list: String) -> Result<(), Box<dyn Error>> {
        let conn = connect_to_db(&self.db_path)?;
        log::debug!("executing query {}", &queries::fetch_collection());
        let mut stmt = conn.prepare(&queries::fetch_collection())?;
        log::info!("checking if lists exists in collection");
        let collection_iter = stmt.query_map([], |row| {
            let list = row.get::<_, String>("name")?;
            Ok(list)
        })?;
        let collection: Vec<_> = collection_iter.filter_map(Result::ok).collect();
        log::debug!("collection {:?}", &collection);
        if !collection.contains(&list) {
            return Err(format!("✘ Can't find list '{list}'").into());
        }
        let dotenv = util::dotenv()?;
        let content = fs::read_to_string(&dotenv)?;
        log::debug!("dotenv contents: {content}");
        let mut new_content = String::new();
        let mut previous = String::from("");
        log::info!("reading .env");
        for line in content.lines() {
            if line.starts_with("CURRENT=") {
                log::info!("updating PREVIOUS to {previous}");
                previous.push_str(line.split('=').next_back().unwrap_or(""));
                log::info!("updating CURRENT to {list}");
                new_content.push_str(format!("CURRENT={list}\n").as_str());
            } else if line.starts_with("PREVIOUS=") {
                new_content.push_str(format!("PREVIOUS={previous}\n").as_str());
            } else {
                new_content.push_str(format!("{line}\n").as_str());
            }
        }
        let mut file = fs::File::create(dotenv)?;
        log::info!("writing back to .env");
        file.write_all(new_content.as_bytes())?;
        println!("✔ Checked out '{list}'");
        Ok(())
    }

    pub fn whoisthis(&self) -> Result<(), Box<dyn Error>> {
        let current = std::env::var("CURRENT")?;
        if current.is_empty() {
            eprintln!("✘ Currently, no list is active");
        } else {
            println!("This is {current}. Ready for duty!");
        }
        Ok(())
    }

    pub fn add(
        &mut self,
        flags: (Option<String>, Option<i64>, Option<String>, Option<String>),
    ) -> Result<(), Box<dyn Error>> {
        let current_list = std::env::var("CURRENT")?;
        log::info!("currently on list {current_list}");
        let current_list_id = util::fetch_active_list_id(&self.db_path)?;
        let conn = util::connect_to_db(&self.db_path)?;
        let (task, prio, due, raw_tag) = flags;
        let tag: Option<Tag> = raw_tag.map(Tag);
        let due_date: Option<Datetime> = match due {
            Some(ref date) => Some(util::parse_date(date)?),
            None => None,
        };
        // logging
        match due_date.as_ref() {
            Some(date) => log::info!("found due date '{}'", date),
            None => log::info!("found due date 'None'"),
        };
        let msg = if let Some(task) = task {
            task
        } else {
            util::edit_in_editor(None)
        };
        log::info!("found task '{}'", msg);
        log::debug!(
            "executing querry `{}`\n with params [{},{},{},{},{}]",
            &queries::add_to_table(&current_list, current_list_id),
            &msg,
            &Status::Open,
            &prio.unwrap_or_default(),
            &due_date.as_ref().unwrap_or(&epoch()),
            &Datetime::new()
        );
        conn.execute(
            &queries::add_to_table(&current_list, current_list_id),
            [
                &msg,
                &Status::Open as &dyn ToSql,
                &prio.unwrap_or_default() as &dyn ToSql,
                &due_date.unwrap_or(epoch()),
                &tag,
                &Datetime::new() as &dyn ToSql,
            ],
        )?;
        Ok(())
    }

    pub fn close(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
        let current = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current);
        let conn = util::connect_to_db(&self.db_path)?;
        log::debug!(
            "executing querry `{}` \n with params [{},{}]",
            &queries::update_status(&current),
            &Status::Closed,
            &id
        );
        conn.execute(&queries::update_status(&current), (&Status::Closed, &id))?;
        Ok(())
    }

    pub fn open(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
        let current = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current);
        let conn = util::connect_to_db(&self.db_path)?;
        log::debug!(
            "executing querry `{}` \n with params [{},{}]",
            &queries::update_status(&current),
            &Status::Open,
            &id
        );
        conn.execute(&queries::update_status(&current), (&Status::Open, &id))?;
        Ok(())
    }

    pub fn delete(&mut self, id: i64) -> Result<(), Box<dyn Error>> {
        let current = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current);
        let conn = util::connect_to_db(&self.db_path)?;
        log::debug!("executing querry {}", queries::delete_task(&current, id));
        conn.execute(&queries::delete_task(&current, id), [])?;
        Ok(())
    }

    pub fn delete_all(&mut self) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let current_list = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current_list);
        log::debug!(
            "executing querry `{}`",
            &queries::fetch_all_ids(&current_list)
        );
        let mut stmt = conn.prepare(&queries::fetch_all_ids(&current_list))?;
        let ids_iter = stmt.query_map([], |row| {
            let id = row.get::<_, i64>("id")?;
            Ok(id)
        })?;
        for id in ids_iter {
            log::debug!(
                "executing querry `{}` \n with params [{}]",
                &queries::delete_by_id(&current_list),
                id.as_ref().unwrap()
            );
            conn.execute(&queries::delete_by_id(&current_list), [&id.unwrap()])?;
        }
        Ok(())
    }

    pub fn reword(&mut self, input: (i64, Option<String>)) -> Result<(), Box<dyn Error>> {
        let conn = util::connect_to_db(&self.db_path)?;
        let current_list = std::env::var("CURRENT")?;
        log::info!("found current list '{}'", &current_list);
        let (id, task) = input;
        let msg = if let Some(task) = task {
            task
        } else {
            let mut stmt = conn.prepare(&queries::fetch_task_by_id(&current_list))?;
            let text: Option<String> = stmt
                .query_row(params![id], |row| row.get::<_, String>("task"))
                .optional()?;
            util::edit_in_editor(text)
        };
        log::info!("found task '{}'", &msg);
        log::debug!(
            "executing querry `{}` \n with params [{},{}]",
            &queries::unpdate_task_by_id(&current_list),
            &id,
            &msg
        );
        conn.execute(&queries::unpdate_task_by_id(&current_list), (&id, &msg))?;
        Ok(())
    }
}
