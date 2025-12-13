use anyhow::Result;
use rusqlite::Connection;

use crate::app::App;
use crate::commands::{Cmd, Plumbing};
use crate::domain::todo::TodoList;
use crate::persistence::{SqlTodoItemRepository, SqlTodoListRepository};
use crate::util;

pub fn run(app: App) -> Result<()> {
    if let Some(cmd) = app.command {
        match Plumbing::try_from(&cmd) {
            Ok(plumbing_cmd) => execute_plumbing_cmd(plumbing_cmd)?,
            Err(_) => execute(cmd)?,
        };
    } else {
        let default = Cmd::default();
        execute(default)?;
    }
    Ok(())
}

fn set_up_repositories(
    conn: &Connection,
) -> Result<(SqlTodoListRepository<'_>, SqlTodoItemRepository<'_>)> {
    let current_list = std::env::var("CURRENT")?;
    log::info!("currently on list {current_list}");
    Ok((
        SqlTodoListRepository::new(conn),
        SqlTodoItemRepository::new(conn, current_list),
    ))
}

fn execute_plumbing_cmd(cmd: Plumbing) -> Result<()> {
    match cmd {
        Plumbing::Init => TodoList::init()?,
        Plumbing::ShowPaths => TodoList::show_paths()?,
        Plumbing::CleanData => TodoList::clean_data()?,
        Plumbing::AutoCompletions { shell } => TodoList::auto_completions(shell),
    }
    Ok(())
}

fn execute(cmd: Cmd) -> Result<()> {
    util::load_env()?;
    let mut todo_list = TodoList::new();
    let conn = util::connect_to_db(&todo_list.db_path)?;
    let (todo_list_repo, todo_item_repo) = set_up_repositories(&conn)?;
    match cmd {
        Cmd::ShowPaths => TodoList::show_paths()?,
        Cmd::CleanData => TodoList::clean_data()?,
        Cmd::Init => TodoList::init()?,
        Cmd::AutoCompletions { shell } => TodoList::auto_completions(shell),
        Cmd::NewList { name, checkout } => {
            todo_list.new_list(&todo_list_repo, &todo_item_repo, name, checkout)?
        }
        Cmd::DeleteList { name } => todo_list.clone().delete_list(&todo_list_repo, name)?,
        Cmd::Load { name } => {
            if name == "-" {
                let previous = std::env::var("PREVIOUS")?;
                todo_list.load(&todo_list_repo, previous)?
            } else {
                todo_list.load(&todo_list_repo, name)?
            }
        }
        Cmd::Whoami => todo_list.whoisthis()?,
        Cmd::Add {
            task,
            due,
            prio,
            tag,
        } => {
            todo_list.add(&todo_item_repo, (task, due, prio, tag))?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::List {
            all,
            done,
            open,
            sort,
            collection,
            tags,
            arg,
        } => match arg {
            Some(arg) if arg.starts_with('@') => {
                todo_list.list_due_date(&todo_item_repo, arg, sort)?
            }
            Some(arg) if arg.starts_with('#') => todo_list.list_tag(&todo_item_repo, arg, sort)?,
            _ => {
                if all {
                    todo_list.list(&todo_item_repo, Some("all".to_string()), sort)?;
                } else if done {
                    todo_list.list(&todo_item_repo, Some("done".to_string()), sort)?;
                } else if open {
                    todo_list.list(&todo_item_repo, Some("open".to_string()), sort)?;
                } else if collection {
                    todo_list.list_collection(&todo_list_repo)?;
                } else if tags {
                    todo_list.list_tags(&todo_item_repo)?;
                } else {
                    todo_list.list(&todo_item_repo, None, sort)?;
                }
            }
        },
        Cmd::Close { ids } => {
            todo_list.close(&todo_item_repo, ids)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::Open { ids } => {
            todo_list.open(&todo_item_repo, ids)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::Delete { id } => todo_list.delete(&todo_item_repo, &id)?,
        Cmd::DeleteAll => todo_list.delete_all(&todo_item_repo)?,
        Cmd::Reword { id, task } => {
            todo_list.reword(&todo_item_repo, &id, task)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::Update {
            ids,
            due,
            prio,
            status,
            tag,
        } => {
            todo_list.update_item(&todo_item_repo, due, prio, status, tag, ids)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::Clear {
            ids,
            due,
            prio,
            tag,
        } => {
            todo_list.clear(&todo_item_repo, ids, due, prio, tag)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::Config => todo_list.clone().config()?,
        Cmd::Show { id } => todo_list.show(&todo_item_repo, &id)?,
    }
    Ok(())
}
