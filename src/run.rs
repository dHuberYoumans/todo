use std::error::Error;

use crate::todo::{Cmd, TodoList};
use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut todo_list = TodoList::new();
    let cmd = config.command;
    match cmd {
        Cmd::Init => todo_list.init()?,
        Cmd::NewList => todo_list.new_list(
            config.args.and_then(|mut arg| arg.pop())
        )?,
        Cmd::DeleteList => todo_list.delete_list(
            config.args.and_then(|mut arg| arg.pop())
        )?,
        Cmd::Load => todo_list.load(
            config.args.and_then(|mut arg| arg.pop())
        )?,
        Cmd::Add => todo_list.add(
            config.args.and_then(|mut arg| arg.pop())
        )?,
        Cmd::List => todo_list.list(
            config.args.and_then(|mut arg| arg.pop())
        )?,
        Cmd::Close => todo_list.close(
            config.args.and_then(|mut arg| arg.pop())
        )?,
        Cmd::Open => todo_list.open(
            config.args.and_then(|mut arg| arg.pop())
        )?,
        Cmd::Delete => todo_list.delete(
            config.args.and_then(|mut arg| arg.pop())
        )?,
        Cmd::DeleteAll => todo_list.delete_all()?,
        Cmd::Reword => todo_list.reword(
            config.args.as_ref().and_then(|args| 
                match (args.get(0), args.get(1)) {
                    (Some(id), Some(task)) => Some((id.clone(), task.clone())),
                        _ => None,
                })
        )?,
        Cmd::Help => todo_list.help(),
        _ => {
            return Err(String::from("✘ Invalid command.").into()); 
        },
    }
    Ok(())
}


