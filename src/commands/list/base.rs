use anyhow::Result;
use colored::{self, Colorize};
use rusqlite::params;
use std::cmp::Reverse;
use tabled::{
    settings::{
        format::{Format, FormatContent},
        object::{Columns, Object, Rows},
        Modify, Style, Width,
    },
    Table,
};

use crate::config::Config;
use crate::domain::{Datetime, Prio, Status, Tag};
use crate::domain::{TodoItem, TodoList, TodoListRepository};
use crate::persistence::schema::epoch;
use crate::util;

impl TodoList {
    pub fn list(
        &mut self,
        repo: &impl TodoListRepository,
        flags: (Option<String>, Option<String>),
    ) -> Result<()> {
        let conn = util::connect_to_db(&self.db_path)?;
        let current_list = std::env::var("CURRENT")?;
        let current_list_id = repo.fetch_id(&current_list)?;
        let id_prefix_length = Config::read()?.style.prefix_id_length;
        let mut sort_key_default = Config::read()?.style.sort_by;
        log::debug!(
            "found current list '{}' with ID={}",
            &current_list,
            &current_list_id
        );
        let opt = flags.0.as_deref().unwrap_or("None");
        log::debug!("using option '{opt}'");
        let query = match opt {
            "--all" => format!("SELECT * FROM {current_list} WHERE list_id = ?"),
            "--done" => format!("SELECT * FROM {current_list} WHERE status=0 AND list_id = ?"),
            _ => format!("SELECT * FROM {current_list} WHERE status=1 AND list_id = ?"),
        };
        log::debug!(
            "executing query `{}` \n with params [{}]",
            &query,
            &current_list_id
        );
        let mut stmt = conn.prepare(&query)?;
        let tasks_iter = stmt.query_map(params![current_list_id], |row| {
            let mut item = TodoItem {
                id: row.get::<_, String>("id")?,
                task: row.get::<_, String>("task")?,
                status: row.get::<_, Status>("status")?,
                prio: row.get::<_, Prio>("prio")?,
                due: row.get::<_, Datetime>("due")?,
                tag: row.get::<_, Tag>("tag")?,
            };
            item.id = item.id.chars().take(id_prefix_length).collect();
            Ok(item)
        })?;
        for task in tasks_iter {
            self.tasks.push(task?);
        }
        if sort_key_default.is_empty() {
            sort_key_default = "id".to_string()
        };
        let sort_key = flags.1.as_deref().unwrap_or(&sort_key_default);
        log::debug!("using sort key {sort_key}");
        match sort_key {
            "id" => self.tasks.sort_by_key(|entry| Reverse(entry.id.clone())),
            "prio" => self.tasks.sort_by_key(|entry| entry.prio),
            "tag" => self.tasks.sort_by_key(|entry| {
                let key = entry.tag.clone();
                (key.0.is_empty(), key)
            }),
            "due" => self.tasks.sort_by_key(|entry| {
                let key = entry.due;
                (key == epoch(), key)
            }),
            _ => self.tasks.sort_by_key(|entry| Reverse(entry.id.clone())),
        };
        let mut table = Table::new(&self.tasks);
        table
            .with(Modify::new(Rows::new(1..).intersect(Columns::single(0))).with(color_id()))
            .with(Modify::new(Rows::new(1..).intersect(Columns::single(2))).with(color_status()))
            .with(Modify::new(Rows::new(1..).intersect(Columns::single(3))).with(color_prio()))
            .with(Modify::new(Columns::single(0)).with(Width::increase(5))) // id
            .with(Modify::new(Columns::single(1)).with(Width::wrap(60))) // task
            .with(Modify::new(Columns::single(2)).with(Width::increase(3))) // status
            .with(Modify::new(Columns::single(3)).with(Width::increase(3))) // prio
            .with(Modify::new(Columns::single(4)).with(Width::increase(3))) // due
            .with(Modify::new(Columns::single(5)).with(Width::wrap(12))) // tag
            .with(Style::modern_rounded());
        println!("{}", table);
        Ok(())
    }
}

fn color_id() -> FormatContent<impl FnMut(&str) -> String + Clone> {
    Format::content(|cell: &str| cell.yellow().to_string())
}

fn color_prio() -> FormatContent<impl FnMut(&str) -> String + Clone> {
    Format::content(|cell: &str| match cell {
        "P1" => cell.red().to_string(),
        "P2" => cell.yellow().to_string(),
        "P3" => cell.green().to_string(),
        _ => cell.to_string(),
    })
}

fn color_status() -> FormatContent<impl FnMut(&str) -> String + Clone> {
    Format::content(|cell: &str| {
        if cell.contains('✔') {
            cell.green().to_string()
        } else {
            cell.red().to_string()
        }
    })
}
