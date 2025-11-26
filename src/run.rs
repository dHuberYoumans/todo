use anyhow::Result;
use rusqlite::Connection;

use crate::commands::{Cmd, Plumbing};
use crate::domain::todo::{Args, TodoList};
use crate::persistence::{SqlTodoItemRepository, SqlTodoListRepository};
use crate::util;

pub fn run(args: Args) -> Result<()> {
    if let Some(cmd) = args.command {
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
        } => todo_list.add(&todo_item_repo, (task, due, prio, tag))?,
        Cmd::List {
            all,
            done,
            sort,
            collection,
            tags,
            arg,
        } => match arg {
            Some(arg) if arg.starts_with('@') => todo_list.list_due_date(&todo_item_repo, arg)?,
            Some(arg) if arg.starts_with('#') => todo_list.list_tag(&todo_item_repo, arg)?,
            _ => {
                if all {
                    todo_list.list(&todo_list_repo, (Some("--all".into()), sort))?;
                } else if done {
                    todo_list.list(&todo_list_repo, (Some("--done".into()), sort))?;
                } else if collection {
                    todo_list.list_collection(&todo_list_repo)?;
                } else if tags {
                    todo_list.list_tags(&todo_item_repo)?;
                } else {
                    todo_list.list(&todo_list_repo, (None, sort))?;
                }
            }
        },
        Cmd::Close { ids } => todo_list.close(&todo_item_repo, ids)?,
        Cmd::Open { ids } => todo_list.open(&todo_item_repo, ids)?,
        Cmd::Delete { id } => todo_list.delete(&todo_item_repo, &id)?,
        Cmd::DeleteAll => todo_list.delete_all(&todo_item_repo)?,
        Cmd::Reword { id, task } => todo_list.reword(&todo_item_repo, &id, task)?,
        Cmd::Update {
            ids,
            due,
            prio,
            status,
            tag,
        } => todo_list.update_item(&todo_item_repo, due, prio, status, tag, ids)?,
        Cmd::Config => todo_list.clone().config()?,
    }
    Ok(())
}
