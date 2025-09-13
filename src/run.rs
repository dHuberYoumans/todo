use std::error::Error;

use crate::todo::{Cmd, TodoList, Args};

pub fn run(args: Args) -> Result<(), Box<dyn Error>>{
    let mut todo_list = TodoList::new();
    match args.command {
        Some(cmd) => match cmd {
            Cmd::Init => todo_list.init()?,
            Cmd::NewList { name, checkout } => { println!("{:?}", checkout); todo_list.new_list( Some(name), Some(checkout) )?},
            Cmd::DeleteList { name } => todo_list.delete_list( Some(name) )?,
            Cmd::Load { name } => todo_list.load( Some(name) )?,
            Cmd::WhoIsThis => todo_list.whoisthis()?,
            Cmd::Add { task, prio, due } => todo_list.add( Some(task), (prio, due) )?,
            Cmd::List { all, done, sort } => {
                if all {
                    todo_list.list((Some("--all".into()), sort))?;
                } else if done {
                    todo_list.list((Some("--done".into()), sort))?;
                } else {
                    todo_list.list((None,sort))?;
                }
            },
            Cmd::Close { id } => todo_list.close( Some(id.to_string()) )?,
            Cmd::Open { id } => todo_list.open( Some(id.to_string()) )?,
            Cmd::Delete { id } => todo_list.delete( Some(id.to_string()) )?,
            Cmd::DeleteAll => todo_list.delete_all()?,
            Cmd::Reword { id, new_task } => todo_list.reword(
                Some((id.to_string(), new_task))
            )?,
        },
        None => todo_list.list((None,None))?
    }
    Ok(())
}


