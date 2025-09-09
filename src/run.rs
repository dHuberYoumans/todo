use std::error::Error;

use crate::todo::{Cmd, TodoList};
use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut todo_list = TodoList::new();
    let cmd = config.command;
    match cmd {
        Cmd::Init => todo_list.init()?,
        Cmd::NewList => todo_list.new_list(
            config.args.unwrap()[0].clone()
        ).unwrap(),
        Cmd::DeleteList => todo_list.delete_list(
            config.args.unwrap()[0].clone()
        ).unwrap(),
        Cmd::Load => todo_list.load(
            config.args.unwrap()[0].clone()
        ).unwrap(),
        Cmd::Add => todo_list.add(
            config.args.unwrap()[0].clone()
        ).unwrap(),
        Cmd::List => todo_list.list(
            config.args.and_then(|mut arg| arg.pop())
        )?,
        Cmd::Open => todo_list.open(
            config.args
                .unwrap()[0]
                .parse::<i64>()?
        ).unwrap(),
        Cmd::Close => todo_list.close(
            config.args
                .unwrap()[0]
                .parse::<i64>()?
        ).unwrap(),
        Cmd::Delete => todo_list.delete(
            config.args
                .unwrap()[0]
                .parse::<i64>()?
        ).unwrap(),
        Cmd::DeleteAll => todo_list.delete_all()?,
        Cmd::Reword => {
            let input = config.args.unwrap();
            todo_list.reword(
                input[0].parse::<i64>()?,
                input[1].clone()
            ).unwrap()},
        Cmd::Help => todo_list.help(),
        _ => {
            return Err(String::from("Invalid command.").into()); 
        },
    }
    Ok(())
}


