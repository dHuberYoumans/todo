use anyhow::Result;
use rusqlite::{named_params, Connection};

use crate::domain::{TodoListCreate, TodoListDelete, TodoListRead, TodoListSchema};

pub struct SqlTodoListRepository<'conn> {
    pub conn: &'conn Connection,
}

impl<'conn> SqlTodoListRepository<'conn> {
    pub const TABLE: &'static str = "collection";

    pub fn new(conn: &'conn Connection) -> Self {
        Self { conn }
    }
}

impl TodoListSchema for SqlTodoListRepository<'_> {
    fn create_table(&self) -> Result<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );",
            Self::TABLE
        );
        self.conn.execute_batch(&sql)?;
        Ok(())
    }
}

impl TodoListCreate for SqlTodoListRepository<'_> {
    fn add(&self, list_name: &str) -> Result<()> {
        let sql = format!("INSERT INTO {} (name) VALUES (:name);", Self::TABLE);
        log::debug!("executing query `{}` with 'name' = {}", &sql, list_name);
        self.conn
            .execute(&sql, named_params! { ":name": list_name, })?;
        Ok(())
    }
}

impl TodoListRead for SqlTodoListRepository<'_> {
    fn fetch_all(&self) -> Result<Vec<String>> {
        let sql = format!("SELECT name FROM {}", Self::TABLE);
        log::debug!("executing query {}", &sql);
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    fn fetch_id(&self, list_name: &str) -> Result<i64> {
        let sql = format!("SELECT id FROM {} WHERE name = (:name);", Self::TABLE);
        log::debug!("executing query {}", &sql);
        self.conn
            .query_row(&sql, named_params! { ":name": list_name }, |row| row.get(0))
            .map_err(Into::into)
    }
}

impl TodoListDelete for SqlTodoListRepository<'_> {
    fn delete(&self, list_name: &str) -> Result<()> {
        let id = self.fetch_id(list_name)?;
        let sql = format!("DELETE FROM {} WHERE id = (:id);", Self::TABLE);
        log::debug!("executing query `{}`", &sql);
        self.conn.execute(&sql, named_params! { ":id": id })?;
        Ok(())
    }
}
