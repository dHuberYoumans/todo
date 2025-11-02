use anyhow::Result;
use rusqlite::{named_params, Connection, OptionalExtension};

use crate::domain::{Datetime, Status, Tag, TodoItem};
use crate::domain::{TodoItemRepository, TodoListRepository};
use crate::persistence::SqlTodoListRepository;

pub struct SqlTodoItemRepository<'conn> {
    pub conn: &'conn Connection,
    pub name: String,
    collection: SqlTodoListRepository<'conn>,
}

impl<'conn> SqlTodoItemRepository<'conn> {
    pub fn new(conn: &'conn Connection, name: String) -> Self {
        Self {
            conn,
            name,
            collection: SqlTodoListRepository::new(conn),
        }
    }
}

impl TodoItemRepository for SqlTodoItemRepository<'_> {
    fn create_table(&self) -> Result<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {table} (
id INTEGER PRIMARY KEY AUTOINCREMENT,
list_id INTEGER NOT NULL 
    REFERENCES {collection}(id)
    ON DELETE CASCADE,
task TEXT NOT NULL,
status INTEGER DEFAULT 0,
prio INTEGER,
due INTEGER,
tag TEXT,
created_at INTEGER
);",
            table = self.name,
            collection = SqlTodoListRepository::TABLE
        );
        log::debug!("executing query `{}`", &sql);
        let _ = self.conn.execute(&sql, [])?;
        Ok(())
    }

    fn add(&self, item: &TodoItem) -> Result<()> {
        let sql = format!(
            "INSERT INTO {} (task, list_id, status, prio, due, tag, created_at)
VALUES (:task, :list_id, :status, :prio, :due, :tag, :created_at);",
            self.name
        );
        let list_id = self.collection.fetch_id(&self.name)?;
        log::debug!("executing query `{}`", &sql);
        let _ = self.conn.execute(
            &sql,
            named_params! {
                ":task": item.task,
                ":list_id": list_id,
                ":status": item.status,
                ":prio": item.prio,
                ":due": item.due,
                ":tag": item.tag,
                ":created_at": Datetime::now(),
            },
        )?;
        Ok(())
    }

    fn fetch_by_due_date(&self, epoch_seconds: i64) -> Result<Vec<TodoItem>> {
        let sql = format!("SELECT * FROM {} WHERE due = :date;", self.name);
        log::debug!("executing query `{}`", &sql);
        let mut stmt = self.conn.prepare(&sql)?;
        let entries = stmt.query_map(named_params! {":date": epoch_seconds}, |row| {
            Ok(TodoItem {
                id: row.get("id")?,
                task: row.get("task")?,
                status: row.get("status")?,
                prio: row.get("prio")?,
                due: row.get("due")?,
                tag: row.get("tag")?,
            })
        })?;
        entries.map(|res| res.map_err(Into::into)).collect()
    }

    fn fetch_by_tag(&self, tag: Tag) -> Result<Vec<TodoItem>> {
        let sql = format!("SELECT * FROM {} WHERE tag=:tag;", self.name);
        log::debug!("executing query `{}`", &sql);
        let mut stmt = self.conn.prepare(&sql)?;
        let entries = stmt.query_map(named_params! { ":tag": tag}, |row| {
            Ok(TodoItem {
                id: row.get("id")?,
                task: row.get("task")?,
                status: row.get("status")?,
                prio: row.get("prio")?,
                due: row.get("due")?,
                tag: row.get("tag")?,
            })
        })?;
        entries.map(|res| res.map_err(Into::into)).collect()
    }

    fn fetch_tags(&self) -> Result<Vec<Tag>> {
        let sql = format!(
            "SELECT DISTINCT tag FROM {} WHERE tag IS NOT NULL;",
            self.name
        );
        log::debug!("executing query `{}`", &sql);
        let mut stmt = self.conn.prepare(&sql)?;
        let tags = stmt
            .query_map([], |row| row.get::<_, Tag>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(tags)
    }

    fn fetch_all_ids(&self) -> Result<Vec<i64>> {
        let sql = format!("SELECT id FROM {};", self.name);
        log::debug!("executing query `{}`", &sql);
        let mut stmt = self.conn.prepare(&sql)?;
        let ids = stmt
            .query_map([], |row| row.get::<_, i64>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ids)
    }

    fn fetch_task_by_id(&self, id: i64) -> Result<Option<String>> {
        let sql = format!("SELECT task FROM {} WHERE id=(:id);", self.name);
        let result = self
            .conn
            .query_row(&sql, named_params! {":id": id}, |row| {
                row.get::<_, String>(0)
            })
            .optional()?;
        Ok(result)
    }

    fn update_task(&self, task: &str, id: i64) -> Result<()> {
        let sql = format!(" UPDATE {} SET task=(:task) WHERE id=(:id);", self.name);
        log::debug!("executing query `{}`", &sql);
        let _ = self
            .conn
            .execute(&sql, named_params! { ":task": task, ":id": id})?;
        Ok(())
    }

    fn update_status(&self, status: Status, id: i64) -> Result<()> {
        let sql = format!("UPDATE {} SET status=(:status) WHERE id=(:id);", self.name);
        log::debug!("executing query `{}`", &sql);
        let _ = self
            .conn
            .execute(&sql, named_params! {":status": status, ":id": id})?;
        Ok(())
    }

    fn delete_task(&self, id: i64) -> Result<()> {
        let sql = format!("DELETE FROM {} WHERE id=(:id);", self.name);
        log::debug!("executing query `{}`", &sql);
        let _ = self.conn.execute(&sql, named_params! {":id": id})?;
        Ok(())
    }
}
