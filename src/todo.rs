use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::error::Error;
use std::cmp::Reverse;
use rusqlite::{Connection, Result, types::ToSql};
use tabled::{Table, settings::Style};
use dirs::home_dir;

use crate::util::{self, Datetime, TodoItem};
use crate::config::Status;

pub enum Cmd {
    Idle,
    Init,
    NewList,
    DeleteList,
    Load,
    Add,
    List,
    Close,
    Open,
    Delete,
    DeleteAll,
    Reword,
    Help,
}


#[derive(Debug, PartialEq, PartialOrd)]
pub struct TodoList{
    pub tasks: Vec<TodoItem>,
    pub db_path: Option<PathBuf>
}

impl TodoList{
    pub fn new() -> Self{
        let db_path = util::get_todo_list_path();
        Self{
            tasks: Vec::new(),
            db_path,
        }
    }

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Initializing..⧖");
        let mut file_path = home_dir().expect("✘ Could not resolve $HOME");
        file_path.push(".todo/.env");
        if file_path.exists() {
            println!("✔︎ Environmental setup found");
            return Ok(());
        }
        println!("Setting up database..⧖");
        fs::create_dir_all(
            file_path
                .parent()
                .unwrap()
        )?;
        let mut env = fs::File::create(file_path)?;
        env.write(b"TODO_DB=todo.db")?;
        self.db_path = util::get_todo_list_path();
        self.new_list(String::from("todo"))?;
        println!("✔︎ Database located at {}", &self.db_path
            .as_ref()
            .map_or(String::from("No path to database found"), |path| path.display().to_string())
            );
        println!("✔︎ All done");
        Ok(())
    }

    pub fn list(&mut self) -> Result<(), Box<dyn Error>>{
        let conn = if let Some(ref path) = &self.db_path {
            Connection::open(path)?
        } else {
            return Err("No path to database found. Consider 'todo init' to initialize a data base".into());
        };
            let mut stmt = conn.prepare("SELECT * FROM tasks")?;
            let tasks_iter = stmt.query_map([], |row| {
                Ok(TodoItem {
                    id: row.get(0)?,
                    task: row.get(1)?,
                    status: row.get(2)?,
                    created_at: row.get(3)?
                })
            })?;
            for task_result in tasks_iter {
                let task = task_result?;
                self.tasks.push(task);
            }
            self.tasks.sort_by_key(|entry| {Reverse(entry.id.clone())});
            let mut table = Table::new(&self.tasks);
            table
                .with(Style::modern_rounded());
            println!("{}", table);
            Ok(())
        }

        pub fn new_list(&self, name: String) -> Result<(), Box<dyn Error>>{
            println!("Creating new_list..⧖");
            let parent = if let Some(ref path) = &self.db_path {
                path.parent()
                    .ok_or("Invalid path to the database")?
            } else {
                return Err("No path to database found. Consider 'todo init' to initialize a data base".into());
            };
            fs::create_dir_all(parent)?;
            let db_file_path = parent.join(format!("{}.db",name));
            let conn = Connection::open(db_file_path)?;
            conn.execute(
                r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task TEXT NOT NULL,
                status INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL
            )"#,
                [])?;
            println!("✔︎ Created new todo list '{}'", name);
            Ok(())
        }

    pub fn delete_list(self, db_name: String) -> Result<(), Box<dyn Error>> {
        let db = if let Some(db_file) = util::get_todo_dir()
            .map( |dir| dir.join(format!("{}.db",db_name))) {
            db_file
        } else {
                return Err(format!("No list named {} found", db_name).into());
        };
        fs::remove_file(db)?;
        println!("✔︎ list '{}' removed", db_name);
        Ok(())
    }

        pub fn load(&mut self, db_name: String) -> Result<(), Box<dyn Error>> {
            let dotenv = if let Some(path) = util::get_env_path() {
                path
            } else {
                return Err("No path to database found. Consider 'todo init' to initialize a data base".into());
            };
            let content = fs::read_to_string(&dotenv)?;
            let mut new_content = String::new();
            for line in content.lines(){
                if line.starts_with("TODO_DB=") {
                    new_content.push_str(format!("TODO_DB={}.db\n",db_name).as_str());
                } else {
                    new_content.push_str(format!("{}\n",line).as_str());
                }
            }
            let mut file = fs::File::create(dotenv)?;
            file.write_all(new_content.as_bytes())?;
            println!("✔︎ Loaded todo list '{}'", db_name);
            Ok(())
        }

        pub fn add(&mut self, task: String) -> Result<(), Box<dyn Error>>{
            let conn = if let Some(ref path) = &self.db_path {
                Connection::open(path)?
            } else {
                return Err("No path to database found. Consider 'todo init' to initialize a data base".into());
            };
            conn.execute(
                "INSERT INTO tasks (task, status, created_at) VALUES (?1, ?2, ?3)",
                (&task, &Status::Closed as &dyn ToSql, &Datetime::new() as &dyn ToSql)
            )?;
            Ok(())
        }

        pub fn close(&mut self, id: i64) -> Result<(), Box<dyn Error>>{
            let conn = if let Some(ref path) = &self.db_path {
                Connection::open(path)?
            } else {
                return Err("No path to database found. Consider 'todo init' to initialize a data base".into());
            };
            conn.execute(
                "UPDATE tasks SET status=0 WHERE id=?1",
                &[&id]
            )?;
            Ok(())
        }

        pub fn open(&mut self, id: i64) -> Result<(), Box<dyn Error>>{
            let conn = if let Some(ref path) = &self.db_path {
                Connection::open(path)?
            } else {
                return Err("No path to database found. Consider 'todo init' to initialize a data base".into());
            };
            conn.execute(
                "UPDATE tasks SET status=1 WHERE id=?1",
                &[&id]
            )?;
            Ok(())
        }

        pub fn delete(&mut self, id: i64) -> Result<(), Box<dyn Error>>{
            let conn = if let Some(ref path) = &self.db_path {
                Connection::open(path)?
            } else {
                return Err("No path to database found. Consider 'todo init' to initialize a data base".into());
            };
            conn.execute(
                "DELETE FROM tasks WHERE id=?1",
                &[&id]
            )?;
            Ok(())
        }

        pub fn delete_all(&mut self) -> Result<(), Box<dyn Error>>{
            let conn = if let Some(ref path) = &self.db_path {
                Connection::open(path)?
            } else {
                return Err("No path to database found. Consider 'todo init' to initialize a data base".into());
            };
            let mut stmt = conn.prepare( "SELECT id FROM tasks")?;
            let ids_iter = stmt.query_map([], |row| {
                let id: i64 = row.get(0)?;
                Ok(id)
            })?;
            for id in ids_iter {
                conn.execute(
                    "DELETE FROM tasks WHERE id=?1",
                    &[&id.unwrap()]
                )?;
            }
            Ok(())
        }

        pub fn reword(&mut self, id: i64, task: String) -> Result<(), Box<dyn Error>>{
            let conn = if let Some(ref path) = &self.db_path {
                Connection::open(path)?
            } else {
                return Err("No path to database found. Consider 'todo init' to initialize a data base".into());
            };
            conn.execute("UPDATE tasks SET task=?2 WHERE id=?1",
                (&id, &task)
            )?;
            Ok(())
        }

        pub fn help(self) -> () {
            let cmds = [
                ("init", "Initialize the cli"),
                ("new-list [name]", "Add a new todo list 'name'"),
                ("delete-list [name]", "Delete the todo list 'name'"),
                ("load [name]", "Load the todo list 'name'"),
                ("add [task]", "Add a new task with the given description"),
                ("list", "Display all tasks in your current todo list"),
                ("open [id]", "Mark the task with the given ID as not done / open"),
                ("close [id]", "Mark the task with the given ID as done / completed"),
                ("delete [id]", "Remove the task with the specified ID"),
                ("delete-all", "Remove all tasks from the todo list"),
                ("reword [id] [new task]", "Replace the description of the task with the given ID"),
            ];
            let max_len = cmds.iter().map(|(cmd,_)| {cmd.len()}).max().unwrap_or(0);
            println!("Todo - A simple todo cli so you don't have to leave the terminal\n");
            for (cmd, desc) in cmds.iter() {
                println!(" {:<width$}    {}", cmd, desc, width=max_len);
            }
        }
    }
