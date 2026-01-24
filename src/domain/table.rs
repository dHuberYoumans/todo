use colored::Colorize;
use tabled::{
    builder::Builder,
    settings::{
        format::{Format, FormatContent},
        object::{Columns, Object, Rows},
        style::Style,
        Modify, Width,
    },
    Table,
};

use crate::domain::table::config::entities::TableStyle;
use crate::domain::{TodoItem, TodoItemRow};
use crate::handlers::config;

pub struct TodoListTable {
    pub table: Table,
}

impl TodoListTable {
    pub fn new(entries: &[TodoItem]) -> Self {
        let mut table = build_table(entries);
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

fn build_table(entries: &[TodoItem]) -> Table {
    let show_due = config::fs::read().map(|c| c.style.show_due).unwrap_or(true);
    let show_tag = config::fs::read().map(|c| c.style.show_tag).unwrap_or(true);
    let mut builder = Builder::default();
    let mut headers = vec!["id", "title", "status", "prio"];
    if show_due {
        headers.push("due")
    };
    if show_tag {
        headers.push("tag")
    };
    builder.push_record(headers);
    let items: Vec<TodoItemRow> = entries.iter().map(TodoItemRow::from).collect();
    for item in items {
        let mut row = vec![
            item.id,
            item.title,
            item.status.to_string(),
            item.prio.to_string(),
        ];
        if show_due {
            row.push(item.due.to_string());
        };
        if show_tag {
            row.push(item.tag.to_string());
        };
        builder.push_record(row);
    }
    builder.build()
}

fn apply_table_style(table: &mut Table) {
    let config = config::fs::read().expect("✘ Couldn't parse the config.");
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
        let id_length = config::fs::read()
            .expect("✘ Couldn't read id")
            .style
            .id_length;
        cell.chars().take(id_length).collect()
    })
}
