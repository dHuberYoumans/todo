use anyhow::{anyhow, Result};
use std::fs;
use std::io::Write;

use crate::domain::{TodoList, TodoListRepository};
use crate::util;

impl TodoList {
    pub fn delete_list(self, repo: &impl TodoListRepository, list: String) -> Result<()> {
        let dotenv = util::dotenv()?;
        let content = fs::read_to_string(&dotenv)?;
        log::debug!("reading env {:?}", dotenv);
        let mut new_content = String::new();
        for line in content.lines() {
            if line.starts_with("CURRENT=") {
                let current_list = line.split('=').next_back().unwrap_or("");
                if list == current_list {
                    return Err(anyhow!(
                        "✘ can't delete the list '{list}' since currently in use"
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
        repo.delete(&list)?;
        println!("✔ List '{list}' removed");
        log::debug!("writing dotenv `{new_content}`");
        let mut file = fs::File::create(dotenv)?;
        file.write_all(new_content.as_bytes())?;
        Ok(())
    }
}
