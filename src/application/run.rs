use anyhow::Result;
use rusqlite::Connection;
use std::path::PathBuf;

use crate::application::handlers::VersionStatus;
use crate::application::{config::Config, handlers};
use crate::cli::app::Cli;
use crate::cli::{Cmd, CompletionsCmd, ListSubCmd, Plumbing};
use crate::domain::{
    grep::GrepOptions,
    update::{ClearOptions, UpdateOptions},
    ListFilters, TodoList,
};
use crate::infrastructure::{self, editor, UserPaths};
use crate::persistence::{connect_to_db, SqlTodoItemRepository, SqlTodoListRepository};

pub fn run(app: Cli, config: &Config) -> Result<()> {
    if let Some(cmd) = app.command {
        match Plumbing::try_from(&cmd) {
            Ok(plumbing_cmd) => execute_plumbing_cmd(plumbing_cmd, config)?,
            Err(_) => execute(cmd, config)?,
        };
    } else {
        let default = Cmd::default();
        execute(default, config)?;
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

fn execute_plumbing_cmd(cmd: Plumbing, config: &Config) -> Result<()> {
    let user_paths = UserPaths::new();
    let config_file = infrastructure::config::get_todo_config(&user_paths)?;
    match cmd {
        Plumbing::Init => handlers::init(),
        Plumbing::ShowPaths => handlers::show_paths(),
        Plumbing::CleanData => {
            let db_file = PathBuf::from(config.database.todo_db.clone());
            handlers::clean_data(config_file, db_file)
        }
        Plumbing::Completions(cmd) => match cmd {
            CompletionsCmd::Generate { shell } => handlers::generate_completions(shell),
            CompletionsCmd::Install { shell } => handlers::install_completions(shell),
        },
    }
}

fn execute(cmd: Cmd, config: &Config) -> Result<()> {
    let user_paths = UserPaths::new();
    infrastructure::env::load_env(&user_paths)?;
    let editor = editor::SysEditor;
    let mut todo_list = TodoList::new();
    let db_path = PathBuf::from(config.database.todo_db.clone());
    let conn = connect_to_db(&db_path)?;
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
            let options = args.into_options(config)?;
            handlers::add(&todo_item_repo, &todo_list, &editor, options)?;
            handlers::list(
                &todo_item_repo,
                &todo_list,
                config,
                None,
                ListFilters::default(),
            )?
        }
        Cmd::List(args) => match args.cmd {
            Some(ListSubCmd::Collection) => handlers::list_collection(&todo_list_repo, &todo_list)?,
            Some(ListSubCmd::Tags) => handlers::list_tags(&todo_item_repo, &todo_list)?,
            None => match args.arg.as_deref() {
                Some(arg) if arg.starts_with('@') => handlers::list_due_date(
                    &todo_item_repo,
                    &todo_list,
                    config,
                    arg.to_string(),
                    args.sort,
                    ListFilters {
                        status: args.status,
                        prio: args.prio,
                    },
                )?,
                Some(arg) if arg.starts_with('#') => handlers::list_tag(
                    &todo_item_repo,
                    &todo_list,
                    config,
                    arg.to_string(),
                    args.sort,
                    ListFilters {
                        status: args.status,
                        prio: args.prio,
                    },
                )?,
                _ => handlers::list(
                    &todo_item_repo,
                    &todo_list,
                    config,
                    args.sort,
                    ListFilters {
                        status: args.status,
                        prio: args.prio,
                    },
                )?,
            },
        },
        Cmd::Close { ids } => {
            handlers::close(&todo_item_repo, &todo_list, ids)?;
            handlers::list(
                &todo_item_repo,
                &todo_list,
                config,
                None,
                ListFilters::default(),
            )?
        }
        Cmd::CloseAll { prio } => {
            todo_list.close_all(&todo_item_repo, prio)?;
            handlers::list(
                &todo_item_repo,
                &todo_list,
                config,
                None,
                ListFilters::default(),
            )?
        }
        Cmd::Open { ids } => {
            handlers::open(&todo_item_repo, &todo_list, ids)?;
            handlers::list(
                &todo_item_repo,
                &todo_list,
                config,
                None,
                ListFilters::default(),
            )?
        }
        Cmd::Delete { id } => handlers::delete(&todo_item_repo, &mut todo_list, &id)?,
        Cmd::DeleteAll => handlers::delete_all(&todo_item_repo, &mut todo_list)?,
        Cmd::Grep(args) => {
            let options = GrepOptions::from(&args);
            handlers::grep(&todo_item_repo, &todo_list, config, &args.pattern, options)?
        }
        Cmd::Reword { id, task } => {
            handlers::reword(&todo_item_repo, &mut todo_list, &editor, &id, task)?;
            handlers::show(&todo_item_repo, &todo_list, &id)?
        }
        Cmd::RND => {
            let rnd_item = handlers::rnd(&todo_item_repo, &todo_list)?;
            if let Some(item) = rnd_item {
                handlers::show(&todo_item_repo, &todo_list, &item.id)?
            }
        }
        Cmd::Update(args) => {
            let options = UpdateOptions::from(&args);
            handlers::update_item(&todo_item_repo, &todo_list, args.ids, options)?;
            handlers::list(
                &todo_item_repo,
                &todo_list,
                config,
                None,
                ListFilters::default(),
            )?
        }
        Cmd::Clear(args) => {
            let options = ClearOptions::from(&args);
            handlers::clear(&todo_item_repo, &todo_list, args.ids, options)?;
            handlers::list(
                &todo_item_repo,
                &todo_list,
                config,
                None,
                ListFilters::default(),
            )?
        }
        Cmd::Upgrade { version, check } => {
            if check {
                let _ = handlers::check_latest_version()?;
            } else {
                let version_status = handlers::check_latest_version()?;
                if version_status == VersionStatus::Behind {
                    handlers::upgrade(version)?
                }
            }
        }
        Cmd::Config => infrastructure::config::edit_config(&editor)?,
        Cmd::Show { id } => handlers::show(&todo_item_repo, &todo_list, &id)?,
        _ => eprintln!("✘ Invalid command"),
    }
    Ok(())
}
