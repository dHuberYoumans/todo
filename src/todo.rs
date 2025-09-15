use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::error::Error;
use std::cmp::Reverse;
use rusqlite::{
    params, Connection, OptionalExtension, Result, ToSql};
use tabled::{
    settings::{
        Modify,
        Style,
        Width,
        object::Columns},
    Table};
use clap::{Parser, Subcommand};

use crate::util::{
    self,
    Status,
    Prio,
    Datetime,
    TodoItem,
    epoch,
    connect_to_db,
};
use crate::paths::UserPaths;
use crate::queries;

#[derive(Parser,Debug)]
#[command(name = "todo", version, about = "A simple todo cli to help you get things done from the comfort of your terminal")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Cmd>,
}

#[derive(Subcommand,Debug)]
pub enum Cmd {
    /// Initialize the cli in CWD  
    Init,
    /// Create a new todo list
    NewList {
        name: String,
        #[arg(long, short='c', help="Directly load new list")]
        checkout: bool,
    },
    /// Delete a todo list
    DeleteList {
        name: String,
    },
    /// Load a todo list
    Load {
        name: String,
    },
    /// Print the name of the todo list in use to stdout
    WhoIsThis,
    /// Add a task
    Add {
        #[arg(long, short='m', help = "Task description")]
        task: Option<String>,
        #[arg(long, short='p', help = "Priority")]
        prio: Option<i64>,
        #[arg(long, short='d', help = "Due date")]
        due: Option<String>
    },
    /// Print the current todo list
    List {
        #[arg(long, help="Show all tasks")]
        all: bool,
        #[arg(long, help="Show all completed tasks")]
        done: bool,
        #[arg(long, short='s', help="Sort tasks")]
        sort: Option<String>,
    },
    /// Mark a task as completed
    Close {
        id: i64,
    },
    /// Open a task
    Open {
        id: i64,
    },
    /// Delete a task
    Delete {
        id: i64,
    },
    /// Delete all tasks in the current todo list
    DeleteAll,
    /// Reword a task
    Reword {
        id: i64,
        #[arg(long, short='m', help = "Task description")]
        task: Option<String>,
    },
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TodoList{
    pub tasks: Vec<TodoItem>,
    pub db_path: Option<PathBuf>
}

impl TodoList{
    pub fn new() -> Self{
        let db_path = util::get_db_path();
        Self{
            tasks: Vec::new(),
            db_path,
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        println!("⧖ Initializing..");
        let user_paths = UserPaths::new();
        let home = user_paths.home;
        let mut file_path = home.to_path_buf();// home_dir().expect("✘ Could not resolve $HOME");
        file_path.push(".todo/.env");
        if file_path.exists() {
            println!("✔︎ Environmental setup found");
            return Ok(());
        }
        println!("⧖ Setting up database..");
        fs::create_dir_all(
            file_path
                .parent()
                .unwrap()
        )?;
        let mut env = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;

        if let Some(config) = user_paths.config {
            writeln!(env, "CONFIG={}", config.to_string_lossy())?
        } else {
            writeln!(env, "CONFIG=")?
        }
        writeln!(env, "CURRENT=todo")?;
        writeln!(env, "PREVIOUS=todo")?;
        self.db_path = util::get_db_path();
        let conn = if let Some(path) = &self.db_path {
            Connection::open(&path)?
        } else{
            return Err(
                "✘ Something went wrong setting up the the database"
                    .to_string()
                    .into()
            )
        };
        conn.execute(&queries::create_collection(), [])?;
        self.new_list(String::from("todo"), false)?;
        println!("✔︎ Database located at {}", &self.db_path
            .as_ref()
            .map_or(String::from("No path to database found"), |path| path.display().to_string())
        );
        println!("✔︎ All done");
        Ok(())
    }

    pub fn list(&mut self, flags: (Option<String>, Option<String>)) -> Result<(), Box<dyn Error>>{
        let conn = connect_to_db(&self.db_path)?;
        let current_list = util::get_active_list_name()?;
        let current_list_id = util::get_active_list_id(&self.db_path, &current_list)?;
        let opt = flags.0;
        let query = match opt.as_deref() {
            Some("--all") => format!("SELECT * FROM {current_list} WHERE list_id = ?"),
            Some("--done") => format!("SELECT * FROM {current_list} WHERE status=0 AND list_id = ?"),
            _ => format!("SELECT * FROM {current_list} WHERE status=1 AND list_id = ?"),
        };
        let mut stmt = conn.prepare(&query)?;
        let tasks_iter = stmt.query_map(params![current_list_id], |row| {
            Ok(TodoItem {
                id: row.get::<_,i64>("id")?,
                task: row.get::<_,String>("task")?,
                status: row.get::<_,Status>("status")?,
                prio: row.get::<_,Prio>("prio")?,
                due: row.get::<_,Datetime>("due")?,
                created_at: row.get::<_,Datetime>("created_at")?
            })
        })?;
        for task_result in tasks_iter {
            let task = task_result?;
            self.tasks.push(task);
        }
        let sort_key = flags.1.unwrap_or("id".to_string());
        match sort_key.as_str() {
            "id" => self.tasks.sort_by_key(|entry| { Reverse(entry.id.clone()) }),
            "prio" => self.tasks.sort_by_key(|entry| { entry.prio.clone() }),
            "due" => self.tasks.sort_by_key(|entry| {
                let key = entry.due.clone();
                (key == epoch(),key)
            }),
            _ => self.tasks.sort_by_key(|entry| { Reverse(entry.id.clone()) }),
        };
        let mut table = Table::new(&self.tasks);
        table
            .with(Modify::new(Columns::single(0)).with(Width::increase(5))) // id
            .with(Modify::new(Columns::single(1)).with(Width::wrap(60))) // task
            .with(Modify::new(Columns::single(2)).with(Width::increase(3))) // status
            .with(Modify::new(Columns::single(3)).with(Width::increase(3))) // prio
            .with(Modify::new(Columns::single(4)).with(Width::increase(3))) // due
            .with(Modify::new(Columns::single(5)).with(Width::increase(12))) // created_at
            .with(Style::modern_rounded());
        println!("{}", table);
        Ok(())
    }

    pub fn new_list(&mut self, list: String, checkout: bool) -> Result<(), Box<dyn Error>>{
        println!("⧖ Creating new_list..");
        let conn = connect_to_db(&self.db_path)?;
        conn.execute( &queries::create_list(&list), [])?;
        println!("✔︎ Created new todo list '{}'", &list);
        conn.execute( &queries::add_to_collection(&list), [])?;
        println!("✔︎ Added {} to collection", &list);
        if checkout {
            self.load(list.clone())?;
            println!("✔︎ Now using '{}'", &list);
        };
        Ok(())
    }

    pub fn delete_list(self, list: String) -> Result<(), Box<dyn Error>> {
        let conn = connect_to_db(&self.db_path)?;
        conn.execute(&queries::delete_list(&list), [])?;
        println!("✔︎ List '{}' removed", &list);
        let dotenv = util::dotenv()?;
        let mut new_content = String::new();
        let content = fs::read_to_string(&dotenv)?;
        let mut current = String::new();
        for line in content.lines().rev() {
            if line.starts_with("PREVIOUS=") {
                current.push_str(
                    line
                        .split('=')
                        .last()
                        .unwrap_or("")
                );
                new_content.push_str(format!("PREVIOUS={}\n", line).as_str());
            } else if line.starts_with("CURRENT=") {
                new_content.push_str(format!("CURRENT={}\n", &current).as_str());
            } else {
                new_content.push_str(format!("PREVIOUS={}\n", line).as_str());
            };
        }
        let mut file = fs::File::create(dotenv)?;
        file.write_all(new_content.as_bytes())?;
        println!("✔︎ Checked out '{}'", &current);
        Ok(())
    }

    pub fn load(&mut self, list: String) -> Result<(), Box<dyn Error>> {
        let dotenv = util::dotenv()?;
        let content = fs::read_to_string(&dotenv)?;
        let mut new_content = String::new();
        let mut previous = String::from("");
        for line in content.lines(){
            if line.starts_with("CURRENT=") {
                previous.push_str(
                    line
                        .split('=')
                        .last()
                        .unwrap_or("")
                );
                new_content.push_str(format!("CURRENT={}\n",&list).as_str());
            } else if line.starts_with("PREVIOUS=") {
                new_content.push_str(format!("PREVIOUS={}\n",&previous).as_str());
            }
            else {
                new_content.push_str(format!("{}\n",line).as_str());
            }
        }
        let mut file = fs::File::create(dotenv)?;
        file.write_all(new_content.as_bytes())?;
        println!("✔︎ Checked out '{}'", &list);
        Ok(())
    }

    pub fn whoisthis(&self) -> Result<(), Box<dyn Error>> {
        let current = util::get_active_list_name()?;
        if current.is_empty() {
            eprintln!("✘ Currently, no list is active");
        } else {
            println!("This is {current}. Ready for duty!");
        }
        Ok(())
    }

    pub fn add(&mut self, flags: (Option<String>, Option<i64>, Option<String>)) -> Result<(), Box<dyn Error>>{
        let current_list = util::get_active_list_name()?;
        let current_list_id = util::get_active_list_id(&self.db_path, &current_list)?;
        let conn = util::connect_to_db(&self.db_path)?;
        let (task, prio, due) = flags;
        let due_date: Option<Datetime> = match due {
            Some(ref date) => Some(util::parse_date(date)?),
            None => None,
        };
        let msg = if let Some(task) = task {
            task
        } else {
            util::edit_in_editor(None)
        };
        conn.execute(
            &queries::add_to_table(
                &current_list,
                current_list_id
            )
            ,[
                &msg,
                &Status::Open as &dyn ToSql,
                &prio.unwrap_or_default() as &dyn ToSql,
                &due_date.unwrap_or(epoch()),
                &Datetime::new() as &dyn ToSql
            ])?;
        Ok(())
    }

    pub fn close(&mut self, id: i64) -> Result<(), Box<dyn Error>>{
        let current = util::get_active_list_name()?;
        let conn = util::connect_to_db(&self.db_path)?;
        conn.execute(
            &queries::update_status(&current),
            (&Status::Closed, &id),
        )?;
        Ok(())
    }

    pub fn open(&mut self, id: i64) -> Result<(), Box<dyn Error>>{
        let current = util::get_active_list_name()?;
        let conn = util::connect_to_db(&self.db_path)?;
        conn.execute(
            &queries::update_status(&current),
            (&Status::Open, &id),
        )?;
        Ok(())
    }

    pub fn delete(&mut self, id: i64) -> Result<(), Box<dyn Error>>{
        let current = util::get_active_list_name()?;
        let conn = util::connect_to_db(&self.db_path)?;
        conn.execute(
            &queries::delete_task(&current, id),
            []
        )?;
        Ok(())
    }

        pub fn delete_all(&mut self) -> Result<(), Box<dyn Error>>{
            let conn = util::connect_to_db(&self.db_path)?;
            let current_list = util::get_active_list_name()?;
            let mut stmt = conn.prepare(&queries::get_all_ids(&current_list))?;
            let ids_iter = stmt.query_map([], |row| {
                let id = row.get::<_,i64>("id")?;
                Ok(id)
            })?;
            for id in ids_iter {
                conn.execute(
                    &queries::delete_by_id(&current_list),
                    &[&id.unwrap()]
                )?;
            }
            Ok(())
        }

    pub fn reword(&mut self, input: (i64, Option<String>)) -> Result<(), Box<dyn Error>>{
        let conn = util::connect_to_db(&self.db_path)?;
        let current_list = util::get_active_list_name()?;
        let (id, task) = input;
        let msg = if let Some(task) = task {
            task
        } else {
            let mut stmt = conn.prepare(&queries::fetch_task_by_id(&current_list))?;
            let text: Option<String> = stmt.query_row(params![id], |row| row.get::<_,String>("task")).optional()?;
            util::edit_in_editor(text)
        };
        conn.execute(&queries::unpdate_task_by_id(&current_list),
            (&id, &msg)
        )?;
        Ok(())
    }

}
