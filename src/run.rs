use anyhow::Result;
use rusqlite::Connection;

use crate::domain::todo::{Args, Cmd, TodoList};
use crate::persistence::{SqlTodoItemRepository, SqlTodoListRepository};
use crate::util;

pub fn run(args: Args) -> Result<()> {
    let mut todo_list = TodoList::new();
    if matches!(args.command, Some(Cmd::Init)) {
        todo_list.init()?;
        return Ok(());
    };

    util::load_env()?;
    let conn = util::connect_to_db(&todo_list.db_path)?;
    let (todo_list_repo, todo_item_repo) = set_up_repositories(&conn)?;

    match args.command {
        Some(cmd) => match cmd {
            Cmd::Init => todo_list.init()?,
            Cmd::NewList { name, checkout } => {
                todo_list.new_list(&todo_list_repo, &todo_item_repo, name, checkout)?
            }
            Cmd::DeleteList { name } => todo_list.delete_list(&todo_list_repo, name)?,
            Cmd::Load { name } => {
                if name == "-" {
                    let previous = std::env::var("PREVIOUS")?;
                    todo_list.load(&todo_list_repo, previous)?
                } else {
                    todo_list.load(&todo_list_repo, name)?
                }
            }
            Cmd::WhoIsThis => todo_list.whoisthis()?,
            Cmd::Add {
                task,
                prio,
                due,
                tag,
            } => todo_list.add(&todo_item_repo, (task, prio, due, tag))?,
            Cmd::List {
                all,
                done,
                sort,
                collection,
                tags,
                arg,
            } => match arg {
                Some(arg) if arg.starts_with('@') => {
                    todo_list.list_due_date(&todo_item_repo, arg)?
                }
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
            Cmd::Close { id } => todo_list.close(&todo_item_repo, id)?,
            Cmd::Open { id } => todo_list.open(&todo_item_repo, id)?,
            Cmd::Delete { id } => todo_list.delete(&todo_item_repo, id)?,
            Cmd::DeleteAll => todo_list.delete_all(&todo_item_repo)?,
            Cmd::Reword { id, task } => todo_list.reword(&todo_item_repo, id, task)?,
            Cmd::Config => todo_list.config()?,
        },
        None => todo_list.list(&todo_list_repo, (None, None))?,
    }
    Ok(())
}

fn set_up_repositories(conn: &Connection) -> Result<(SqlTodoListRepository<'_>, SqlTodoItemRepository<'_>)> {
    let current_list = std::env::var("CURRENT")?;
    log::info!("currently on list {current_list}");
    Ok( (SqlTodoListRepository::new(&conn), SqlTodoItemRepository::new(&conn, current_list)) 
    )
}
