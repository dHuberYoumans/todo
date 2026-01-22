use anyhow::Result;
use rusqlite::Connection;

use crate::adapters::cli;
use crate::app::App;
use crate::commands::{upgrade::check_latest_version, Cmd, CompletionsCmd, Plumbing};
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
        Plumbing::Completions(cmd) => match cmd {
            CompletionsCmd::Generate { shell } => TodoList::generate_completions(shell)?,
            CompletionsCmd::Install { shell } => TodoList::install_completions(shell)?,
        },
    }
    Ok(())
}

fn execute(cmd: Cmd) -> Result<()> {
    util::load_env()?;
    let mut todo_list = TodoList::new();
    let conn = util::connect_to_db(&todo_list.db_path)?;
    let (todo_list_repo, todo_item_repo) = set_up_repositories(&conn)?;
    match cmd {
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
        Cmd::Whoami => cli::whoami()?,
        Cmd::Add(args) => {
            cli::add(&todo_list, &todo_item_repo, args)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::List(args) => match args.arg.as_deref() {
            Some(arg) if arg.starts_with('@') => {
                todo_list.list_due_date(&todo_item_repo, arg.to_string(), args.sort, args.filter)?
            }
            Some(arg) if arg.starts_with('#') => {
                todo_list.list_tag(&todo_item_repo, arg.to_string(), args.sort, args.filter)?
            }
            _ => {
                if args.collection {
                    todo_list.list_collection(&todo_list_repo)?;
                } else if args.tags {
                    todo_list.list_tags(&todo_item_repo)?;
                } else {
                    todo_list.list(&todo_item_repo, args.sort, args.filter)?;
                }
            }
        },
        Cmd::Close { ids } => {
            cli::close(&todo_item_repo, &todo_list, ids)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::CloseAll { prio } => {
            todo_list.close_all(&todo_item_repo, prio)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::Open { ids } => {
            cli::open(&todo_item_repo, &todo_list, ids)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::Delete { id } => cli::delete(&todo_item_repo, &mut todo_list, &id)?,
        Cmd::DeleteAll => todo_list.delete_all(&todo_item_repo)?,
        Cmd::Reword { id, task } => {
            cli::reword(&todo_item_repo, &mut todo_list, &id, task)?;
            todo_list.show(&todo_item_repo, &id)?
        }
        Cmd::Update {
            ids,
            due,
            prio,
            status,
            tag,
        } => {
            cli::update_item(&todo_item_repo, &todo_list, due, prio, status, tag, ids)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::Clear {
            ids,
            due,
            prio,
            tag,
        } => {
            cli::clear(&todo_item_repo, &todo_list, ids, due, prio, tag)?;
            todo_list.list(&todo_item_repo, None, None)?
        }
        Cmd::Upgrade { version, check } => {
            if check {
                check_latest_version()?
            } else {
                todo_list.upgrade(version)?
            }
        }
        Cmd::Config => todo_list.clone().config()?,
        Cmd::Show { id } => todo_list.show(&todo_item_repo, &id)?,
        _ => eprintln!("✘ invalid command"),
    }
    Ok(())
}
