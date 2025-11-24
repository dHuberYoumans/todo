use colored::Colorize;
use tabled::{
    settings::{
        format::{Format, FormatContent},
        object::{Columns, Object, Rows},
        style::Style,
        Modify, Width,
    },
    Table,
};

use crate::config::{Config, TableStyle};
use crate::domain::TodoItem;

pub struct TodoListTable {
    pub table: Table,
}

impl TodoListTable {
    pub fn new(entries: &Vec<TodoItem>) -> Self {
        let mut table = Table::new(entries);
        table
            .with(Modify::new(Rows::new(1..).intersect(Columns::single(0))).with(format_id()))
            .with(Modify::new(Rows::new(1..).intersect(Columns::single(0))).with(color_id()))
            .with(Modify::new(Rows::new(1..).intersect(Columns::single(2))).with(color_status()))
            .with(Modify::new(Rows::new(1..).intersect(Columns::single(3))).with(color_prio()))
            .with(Modify::new(Columns::single(0)).with(Width::increase(5))) // id
            .with(Modify::new(Columns::single(1)).with(Width::wrap(60))) // task
            .with(Modify::new(Columns::single(2)).with(Width::increase(3))) // status
            .with(Modify::new(Columns::single(3)).with(Width::increase(3))) // prio
            .with(Modify::new(Columns::single(4)).with(Width::increase(3))) // due
            .with(Modify::new(Columns::single(5)).with(Width::wrap(12))); // tag
        apply_table_style(&mut table);
        Self { table }
    }
    pub fn print(&self) {
        println!("{}", self.table);
    }
}

fn apply_table_style(table: &mut Table) {
    let config = Config::read().expect("✘ Couldn't parse the config.");
    let style = config.style.table.into();
    match style {
        TableStyle::Ascii => table.with(Style::ascii()),
        TableStyle::AsciiRounded => table.with(Style::ascii_rounded()),
        TableStyle::Modern => table.with(Style::modern()),
        TableStyle::ModernRounded => table.with(Style::modern_rounded()),
        TableStyle::Markdown => table.with(Style::markdown()),
    };
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
            cell.to_string()
        }
    })
}

fn format_id() -> FormatContent<impl FnMut(&str) -> String + Clone> {
    Format::content(|cell: &str| {
        let id_length = Config::read().expect("✘ Couldn't read id").style.id_length;
        cell.chars().take(id_length).collect()
    })
}
