use std::error::Error;

use crate::domain::todo::{Args, Cmd, TodoList};

pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let mut todo_list = TodoList::new();
    match args.command {
        Some(cmd) => match cmd {
            Cmd::Init => todo_list.init()?,
            Cmd::NewList { name, checkout } => todo_list.new_list(name, checkout)?,
            Cmd::DeleteList { name } => todo_list.delete_list(name)?,
            Cmd::Load { name } => {
                if name == "-" {
                    let previous = std::env::var("PREVIOUS")?;
                    todo_list.load(previous)?
                } else {
                    todo_list.load(name)?
                }
            }
            Cmd::WhoIsThis => todo_list.whoisthis()?,
            Cmd::Add {
                task,
                prio,
                due,
                tag,
            } => todo_list.add((task, prio, due, tag))?,
            Cmd::List {
                all,
                done,
                sort,
                collection,
                tags,
                arg,
            } => match arg {
                Some(arg) if arg.starts_with('@') => todo_list.list_due_date(arg)?,
                Some(arg) if arg.starts_with('#') => todo_list.list_tag(arg)?,
                _ => {
                    if all {
                        todo_list.list((Some("--all".into()), sort))?;
                    } else if done {
                        todo_list.list((Some("--done".into()), sort))?;
                    } else if collection {
                        todo_list.list_collection()?;
                    } else if tags {
                        todo_list.list_tags()?;
                    } else {
                        todo_list.list((None, sort))?;
                    }
                }
            },
            Cmd::Close { id } => todo_list.close(id)?,
            Cmd::Open { id } => todo_list.open(id)?,
            Cmd::Delete { id } => todo_list.delete(id)?,
            Cmd::DeleteAll => todo_list.delete_all()?,
            Cmd::Reword { id, task } => todo_list.reword((id, task))?,
            Cmd::Config => todo_list.config()?,
        },
        None => todo_list.list((None, None))?,
    }
    Ok(())
}
