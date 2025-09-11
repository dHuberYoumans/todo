use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::error::Error;
use std::cmp::Reverse;
use rusqlite::{Connection, Result, types::ToSql};
use tabled::{settings::{Modify, Style, Width, object::Columns}, Table};
use dirs::home_dir;
use clap::{Parser, Subcommand};

use crate::util::{self, Status, Datetime, TodoItem};

#[derive(Parser,Debug)]
#[command(name = "todo", version, about = "A simple todo cli to help you get things done from the comfort of your terminal")]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Cmd>,
}

#[derive(Subcommand,Debug)]
pub enum Cmd {
    Init,
    NewList {
        name: String,
    },
    DeleteList {
        name: String,
    },
    Load {
        name: String,
    },
    Add {
        task: String,
    },
    List {
        #[arg(long, help = "Show all tasks")]
        all: bool,
        #[arg(long, help = "Show all completed tasks")]
        done: bool,
    },
    Close {
        id: i64,
    },
    Open {
        id: i64,
    },
    Delete {
        id: i64,
    },
    DeleteAll,
    Reword {
        id: i64,
        new_task: String,
    },
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
        self.new_list(Some(String::from("todo")))?;
        println!("✔︎ Database located at {}", &self.db_path
            .as_ref()
            .map_or(String::from("No path to database found"), |path| path.display().to_string())
            );
        println!("✔︎ All done");
        Ok(())
    }

    pub fn list(&mut self, flag: Option<String>) -> Result<(), Box<dyn Error>>{
        let conn = if let Some(ref path) = &self.db_path {
            Connection::open(path)?
        } else {
            return Err("No path to database found. Consider 'todo init' to initialize a data base".into());
        };
        let mut stmt = match flag {
            Some(opt) if opt =="--all"  => conn.prepare("SELECT * FROM tasks")?,
            Some(opt) if opt == "--done" => conn.prepare("SELECT * FROM tasks WHERE status=0")?,
            _ => conn.prepare("SELECT * FROM tasks WHERE status=1")?,
        };
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
                .with(Modify::new(Columns::single(0)).with(Width::increase(5))) // id
                .with(Modify::new(Columns::single(1)).with(Width::wrap(60))) // task
                .with(Modify::new(Columns::single(2)).with(Width::increase(3))) // status
                .with(Modify::new(Columns::single(3)).with(Width::increase(12))) // created_at
                .with(Style::modern_rounded());
            println!("{}", table);
            Ok(())
        }

    pub fn new_list(&self, list: Option<String>) -> Result<(), Box<dyn Error>>{
        let name = if let Some(list) = list {
            list
        } else {
            return Err(
                "✘ Missing argument. Please provide a name for your new list."
                .to_string()
                .into()
            );
        };
        println!("Creating new_list..⧖");
        let parent = if let Some(ref path) = &self.db_path {
            path.parent()
                .ok_or("✘ Invalid path to the database")?
        } else {
            return Err("✘ No path to database found. Consider 'todo init' to initialize a data base".into());
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

    pub fn delete_list(self, list: Option<String>) -> Result<(), Box<dyn Error>> {
        let db_name = if let Some(list) = list {
            list
        } else {
            return Err(
                "✘ Missing argument. Please chose the list you want to delete."
                .to_string()
                .into()
            );
        };
        let db = if let Some(db_file) = util::get_todo_dir()
            .map(|dir| dir.join(format!("{}.db", db_name))) {
            db_file
        } else {
                return Err(
                format!("✘ No list named {} found", db_name)
                    .into()
            );
        };
        fs::remove_file(db)?;
        println!("✔︎ list '{}' removed", db_name);
        Ok(())
    }

    pub fn load(&mut self, list: Option<String>) -> Result<(), Box<dyn Error>> {
        let db_name = if let Some(list) = list {
            list
        } else {
            return Err(
                "✘ Missing argument. Please provide the list you want to load"
                    .to_string()
                    .into()
            );
        };
        let dotenv = if let Some(path) = util::get_env_path() {
            path
        } else {
            return Err("✘ No path to database found. Consider 'todo init' to initialize a data base".into());
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

    pub fn add(&mut self, task: Option<String>) -> Result<(), Box<dyn Error>>{
        let conn = if let Some(ref path) = &self.db_path {
            Connection::open(path)?
        } else {
            return Err("✘ No path to database found. Consider 'todo init' to initialize a data base".into());
        };
        if let Some(task) = task {
            conn.execute(
                "INSERT INTO tasks (task, status, created_at) VALUES (?1, ?2, ?3)",
                (&task, &Status::Closed as &dyn ToSql, &Datetime::new() as &dyn ToSql)
            )?; 
        } else {
            return Err(
                "✘ Missing argument. Please specify the task you want to add"
                    .to_string()
                    .into()
            );
        }
        Ok(())
    }

    pub fn close(&mut self, id: Option<String>) -> Result<(), Box<dyn Error>>{
        let conn = if let Some(ref path) = &self.db_path {
            Connection::open(path)?
        } else {
            return Err("✘ No path to database found. Consider 'todo init' to initialize a data base".into());
        };
        if let Some(id) = id {
            let task_id = id.parse::<i64>()?;
            conn.execute(
                "UPDATE tasks SET status=0 WHERE id=?1",
                &[&task_id]
            )?;
        } else {
            return Err(
                "✘ Missing argument. Please specify the id of the task you want to close."
                    .to_string()
                    .into()
            );
        };
        Ok(())
    }

    pub fn open(&mut self, id: Option<String>) -> Result<(), Box<dyn Error>>{
        let conn = if let Some(ref path) = &self.db_path {
            Connection::open(path)?
        } else {
            return Err("✘ No path to database found. Consider 'todo init' to initialize a data base".into());
        };
        if let Some(id) = id {
            let task_id = id.parse::<i64>()?;
            conn.execute(
                "UPDATE tasks SET status=1 WHERE id=?1",
                &[&task_id]
            )?;
        } else {
            return Err(
                "✘ Missing argument. Please specify the id of the task you want to open."
                    .to_string()
                    .into()
            );
        }
        Ok(())
    }

    pub fn delete(&mut self, id: Option<String>) -> Result<(), Box<dyn Error>>{
        let conn = if let Some(ref path) = &self.db_path {
            Connection::open(path)?
        } else {
            return Err("✘ No path to database found. Consider 'todo init' to initialize a data base".into());
        };
        if let Some(id) = id {
            let task_id = id.parse::<i64>()?;
            conn.execute(
                "DELETE FROM tasks WHERE id=?1",
                &[&task_id]
            )?;
        } else {
            return Err(
                "✘ Missing argument. Please specify the id of the task you want to delete."
                    .to_string()
                    .into()
            );
        }
        Ok(())
    }

        pub fn delete_all(&mut self) -> Result<(), Box<dyn Error>>{
            let conn = if let Some(ref path) = &self.db_path {
                Connection::open(path)?
            } else {
                return Err("✘ No path to database found. Consider 'todo init' to initialize a data base".into());
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

    pub fn reword(&mut self, input: Option<(String,String)>) -> Result<(), Box<dyn Error>>{
        let conn = if let Some(ref path) = &self.db_path {
            Connection::open(path)?
        } else {
            return Err("✘ No path to database found. Consider 'todo init' to initialize a data base".into());
        };
        if let Some((id, task)) = input {
            let task_id = id.parse::<i64>()?;
            conn.execute("UPDATE tasks SET task=?2 WHERE id=?1",
                (&task_id, &task)
            )?;
        } else {
            return Err(
                "✘ Missing argument. Please specify the id and the new description of the task."
                    .to_string()
                    .into()
            );
        }
        Ok(())
    }

}
