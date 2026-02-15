use anyhow::Result;

use crate::domain::{grep::GrepOptions, TodoItemRead, TodoList, TodoListTable};

use crate::application::config::Config;

pub fn grep<R>(
    repo: &R,
    todo_list: &TodoList,
    config: &Config,
    pattern: &str,
    options: GrepOptions,
) -> Result<()>
where
    R: TodoItemRead,
{
    let matches = todo_list.grep(repo, pattern, options)?;
    if matches.is_empty() {
        println!("ℹ No match found");
    } else {
        let table = TodoListTable::new(&matches, config);
        table.print();
    }
    Ok(())
}
