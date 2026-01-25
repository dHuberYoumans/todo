use anyhow::{anyhow, Result};
use std::fs;
use std::io::Write;

use crate::domain::{TodoList, TodoListRepository};
use crate::infrastructure::{env, UserPaths};

pub fn delete_list(
    repo: &impl TodoListRepository,
    todo_list: &TodoList,
    list: String,
) -> Result<()> {
    let user_paths = UserPaths::new();
    let dotenv = env::dotenv(&user_paths);
    let content = fs::read_to_string(&dotenv)?;
    log::debug!("reading env {:?}", dotenv);
    let mut new_content = String::new();
    for line in content.lines() {
        if line.starts_with("CURRENT=") {
            let current_list = line.split('=').next_back().unwrap_or("");
            if list == current_list {
                return Err(anyhow!(
                    "✘ Can't delete the list '{list}' since currently in use"
                ));
            };
            new_content.push_str(&format!("{line}\n"));
        } else if line.starts_with("PREVIOUS=") {
            let current_list = line.split('=').next_back().unwrap_or("");
            if list == current_list {
                new_content.push_str("PREVIOUS=\n");
            } else {
                new_content.push_str(&format!("{line}\n"));
            };
        } else {
            new_content.push_str(&format!("{line}\n"));
        }
    }
    println!("▶ Removing list '{list}'...");
    todo_list.delete_list(repo, &list)?;
    println!("✔ Done");
    log::debug!("writing dotenv `{new_content}`");
    let mut file = fs::File::create(dotenv)?;
    file.write_all(new_content.as_bytes())?;
    Ok(())
}
