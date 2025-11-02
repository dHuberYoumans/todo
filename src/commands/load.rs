use anyhow::{anyhow, Result};
use std::fs;
use std::io::Write;

use crate::domain::{TodoList, TodoListRepository};
use crate::util;

impl TodoList {
    pub fn load(&mut self, repo: &impl TodoListRepository, list: String) -> Result<()> {
        let collection = repo.fetch_all()?;
        log::info!("checking if lists exists in collection");
        log::debug!("collection {:?}", &collection);
        if !collection.contains(&list) {
            return Err(anyhow!("✘ Can't find list '{list}'"));
        }
        let dotenv = util::dotenv()?;
        let content = fs::read_to_string(&dotenv)?;
        log::debug!("dotenv contents: {content}");
        let mut new_content = String::new();
        let mut previous = String::from("");
        log::info!("reading .env");
        for line in content.lines() {
            if line.starts_with("CURRENT=") {
                log::info!("updating PREVIOUS to {previous}");
                previous.push_str(line.split('=').next_back().unwrap_or(""));
                log::info!("updating CURRENT to {list}");
                new_content.push_str(format!("CURRENT={list}\n").as_str());
            } else if line.starts_with("PREVIOUS=") {
                new_content.push_str(format!("PREVIOUS={previous}\n").as_str());
            } else {
                new_content.push_str(format!("{line}\n").as_str());
            }
        }
        let mut file = fs::File::create(dotenv)?;
        log::info!("writing back to .env");
        file.write_all(new_content.as_bytes())?;
        println!("✔ Checked out '{list}'");
        Ok(())
    }
}
