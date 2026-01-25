use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

use crate::application::app::App;
use crate::application::{self, handlers};
use crate::domain::{Cmd, CompletionsCmd, Plumbing, TodoList};
use crate::infrastructure::{self, editor, UserPaths};
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
    let user_paths = UserPaths::new();
    let config_file = infrastructure::config::get_todo_config(&user_paths)?;
    match cmd {
        Plumbing::Init => handlers::init(),
        Plumbing::ShowPaths => handlers::show_paths(),
        Plumbing::CleanData => {
            let config = application::config::load_config()?;
            let db_file = PathBuf::from(config.database.todo_db);
            handlers::clean_data(config_file, db_file)
        }
        Plumbing::Completions(cmd) => match cmd {
            CompletionsCmd::Generate { shell } => handlers::generate_completions(shell),
            CompletionsCmd::Install { shell } => handlers::install_completions(shell),
        },
    }
}

fn execute(cmd: Cmd) -> Result<()> {
    util::load_env()?;
    let editor = editor::SysEditor;
    let mut todo_list = TodoList::new();
    let config = application::config::load_config()?;
    let db_path = PathBuf::from(&config.database.todo_db);
    let conn = util::connect_to_db(&db_path)?;
    let (todo_list_repo, todo_item_repo) = set_up_repositories(&conn)?;
    match cmd {
        Cmd::NewList { name, checkout } => {
            handlers::new_list(&todo_list_repo, &todo_item_repo, &todo_list, &name)?;
            if checkout {
                log::info!("checking out list '{}'", &name);
                handlers::load(&todo_list_repo, &mut todo_list, &name)?;
                println!("✔ Now using '{}'", &name);
            };
        }
        Cmd::DeleteList { name } => handlers::delete_list(&todo_list_repo, &todo_list, name)?,
        Cmd::Load { name } => {
            if name == "-" {
                let previous = std::env::var("PREVIOUS")?;
                handlers::load(&todo_list_repo, &mut todo_list, &previous)?
            } else {
                handlers::load(&todo_list_repo, &mut todo_list, &name)?
            }
        }
        Cmd::Whoami => handlers::whoami()?,
        Cmd::Add(args) => {
            handlers::add(&todo_item_repo, &todo_list, &config, &editor, args)?;
            handlers::list(&todo_item_repo, &todo_list, &config, None, None)?
        }
        Cmd::List(args) => match args.arg.as_deref() {
            Some(arg) if arg.starts_with('@') => handlers::list_due_date(
                &todo_item_repo,
                &todo_list,
                &config,
                arg.to_string(),
                args.sort,
                args.filter,
            )?,
            Some(arg) if arg.starts_with('#') => handlers::list_tag(
                &todo_item_repo,
                &todo_list,
                &config,
                arg.to_string(),
                args.sort,
                args.filter,
            )?,
            _ => {
                if args.collection {
                    handlers::list_collection(&todo_list_repo, &todo_list)?;
                } else if args.tags {
                    handlers::list_tags(&todo_item_repo, &todo_list)?;
                } else {
                    handlers::list(&todo_item_repo, &todo_list, &config, args.sort, args.filter)?;
                }
            }
        },
        Cmd::Close { ids } => {
            handlers::close(&todo_item_repo, &todo_list, ids)?;
            handlers::list(&todo_item_repo, &todo_list, &config, None, None)?
        }
        Cmd::CloseAll { prio } => {
            todo_list.close_all(&todo_item_repo, prio)?;
            handlers::list(&todo_item_repo, &todo_list, &config, None, None)?
        }
        Cmd::Open { ids } => {
            handlers::open(&todo_item_repo, &todo_list, ids)?;
            handlers::list(&todo_item_repo, &todo_list, &config, None, None)?
        }
        Cmd::Delete { id } => handlers::delete(&todo_item_repo, &mut todo_list, &id)?,
        Cmd::DeleteAll => handlers::delete_all(&todo_item_repo, &mut todo_list)?,
        Cmd::Reword { id, task } => {
            handlers::reword(&todo_item_repo, &mut todo_list, &editor, &id, task)?;
            handlers::show(&todo_item_repo, &todo_list, &id)?
        }
        Cmd::Update {
            ids,
            due,
            prio,
            status,
            tag,
        } => {
            handlers::update_item(&todo_item_repo, &todo_list, due, prio, status, tag, ids)?;
            handlers::list(&todo_item_repo, &todo_list, &config, None, None)?
        }
        Cmd::Clear {
            ids,
            due,
            prio,
            tag,
        } => {
            handlers::clear(&todo_item_repo, &todo_list, ids, due, prio, tag)?;
            handlers::list(&todo_item_repo, &todo_list, &config, None, None)?
        }
        Cmd::Upgrade { version, check } => {
            if check {
                handlers::check_latest_version()?
            } else {
                handlers::upgrade(version)?
            }
        }
        Cmd::Config => infrastructure::config::edit_config(&editor)?,
        Cmd::Show { id } => handlers::show(&todo_item_repo, &todo_list, &id)?,
        _ => eprintln!("✘ Invalid command"),
    }
    Ok(())
}
