use anyhow::Result;
use rusqlite::Connection;

use crate::adapters::cli;
use crate::app::App;
use crate::domain::{Cmd, CompletionsCmd, Plumbing, TodoList};
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
        Plumbing::Init => cli::init(),
        Plumbing::ShowPaths => cli::show_paths(),
        Plumbing::CleanData => cli::clean_data(),
        Plumbing::Completions(cmd) => match cmd {
            CompletionsCmd::Generate { shell } => cli::generate_completions(shell),
            CompletionsCmd::Install { shell } => cli::install_completions(shell),
        },
    }
}

fn execute(cmd: Cmd) -> Result<()> {
    util::load_env()?;
    let mut todo_list = TodoList::new();
    let conn = util::connect_to_db(&todo_list.db_path)?;
    let (todo_list_repo, todo_item_repo) = set_up_repositories(&conn)?;
    match cmd {
        Cmd::NewList { name, checkout } => {
            cli::new_list(&todo_list_repo, &todo_item_repo, &todo_list, &name)?;
            if checkout {
                log::info!("checking out list '{}'", &name);
                cli::load(&todo_list_repo, &mut todo_list, &name)?;
                println!("✔ Now using '{}'", &name);
            };
        }
        Cmd::DeleteList { name } => cli::delete_list(&todo_list_repo, &todo_list, name)?,
        Cmd::Load { name } => {
            if name == "-" {
                let previous = std::env::var("PREVIOUS")?;
                cli::load(&todo_list_repo, &mut todo_list, &previous)?
            } else {
                cli::load(&todo_list_repo, &mut todo_list, &name)?
            }
        }
        Cmd::Whoami => cli::whoami()?,
        Cmd::Add(args) => {
            cli::add(&todo_list, &todo_item_repo, args)?;
            cli::list(&todo_item_repo, &todo_list, None, None)?
        }
        Cmd::List(args) => match args.arg.as_deref() {
            Some(arg) if arg.starts_with('@') => cli::list_due_date(
                &todo_item_repo,
                &todo_list,
                arg.to_string(),
                args.sort,
                args.filter,
            )?,
            Some(arg) if arg.starts_with('#') => cli::list_tag(
                &todo_item_repo,
                &todo_list,
                arg.to_string(),
                args.sort,
                args.filter,
            )?,
            _ => {
                if args.collection {
                    cli::list_collection(&todo_list_repo, &todo_list)?;
                } else if args.tags {
                    cli::list_tags(&todo_item_repo, &todo_list)?;
                } else {
                    cli::list(&todo_item_repo, &todo_list, args.sort, args.filter)?;
                }
            }
        },
        Cmd::Close { ids } => {
            cli::close(&todo_item_repo, &todo_list, ids)?;
            cli::list(&todo_item_repo, &todo_list, None, None)?
        }
        Cmd::CloseAll { prio } => {
            todo_list.close_all(&todo_item_repo, prio)?;
            cli::list(&todo_item_repo, &todo_list, None, None)?
        }
        Cmd::Open { ids } => {
            cli::open(&todo_item_repo, &todo_list, ids)?;
            cli::list(&todo_item_repo, &todo_list, None, None)?
        }
        Cmd::Delete { id } => cli::delete(&todo_item_repo, &mut todo_list, &id)?,
        Cmd::DeleteAll => cli::delete_all(&todo_item_repo, &mut todo_list)?,
        Cmd::Reword { id, task } => {
            cli::reword(&todo_item_repo, &mut todo_list, &id, task)?;
            cli::show(&todo_item_repo, &todo_list, &id)?
        }
        Cmd::Update {
            ids,
            due,
            prio,
            status,
            tag,
        } => {
            cli::update_item(&todo_item_repo, &todo_list, due, prio, status, tag, ids)?;
            cli::list(&todo_item_repo, &todo_list, None, None)?
        }
        Cmd::Clear {
            ids,
            due,
            prio,
            tag,
        } => {
            cli::clear(&todo_item_repo, &todo_list, ids, due, prio, tag)?;
            cli::list(&todo_item_repo, &todo_list, None, None)?
        }
        Cmd::Upgrade { version, check } => {
            if check {
                cli::check_latest_version()?
            } else {
                cli::upgrade(version)?
            }
        }
        Cmd::Config => cli::config::edit()?,
        Cmd::Show { id } => cli::show(&todo_item_repo, &todo_list, &id)?,
        _ => eprintln!("✘ Invalid command"),
    }
    Ok(())
}
