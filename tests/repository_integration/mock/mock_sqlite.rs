use anyhow::Result;
use rusqlite::Connection;
use todo::domain::{Datetime, Prio, Status, Tag, TodoItem};
use todo::persistence::{SqlTodoItemRepository, SqlTodoListRepository};

pub struct MockSqlDb {
    pub conn: Connection,
}

impl MockSqlDb {
    pub fn new() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS collection (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY UNIQUE,
            list_id INTEGER NOT NULL 
                REFERENCES collection(id)
                ON DELETE CASCADE,
            task TEXT NOT NULL,
            status INTEGER DEFAULT 0,
            prio INTEGER,
            due INTEGER,
            tag TEXT,
            created_at INTEGER
            );

            INSERT INTO collection (name) VALUES ('todos');
            "#,
        )?;
        Ok(Self { conn })
    }
}

pub struct MockItemEnv {
    pub db: MockSqlDb,
}

impl MockItemEnv {
    pub fn new() -> Result<Self> {
        let db = MockSqlDb::new()?;
        Ok(Self { db })
    }

    pub fn repo(&self, list_name: impl Into<String>) -> SqlTodoItemRepository<'_> {
        SqlTodoItemRepository::new(&self.db.conn, list_name.into())
    }
}

#[derive(Debug)]
pub struct MockTodoItem {
    pub item: TodoItem,
}

impl MockTodoItem {
    pub fn new(
        id: String,
        msg: impl Into<String>,
        prio: Option<Prio>,
        due: Option<Datetime>,
        tag: Option<Tag>,
    ) -> Self {
        Self {
            item: TodoItem {
                id,
                task: msg.into(),
                status: Status::Open,
                due: due.unwrap_or_default(),
                prio: prio.unwrap_or_default(),
                tag: tag.unwrap_or_default(),
            },
        }
    }
}

impl Default for MockTodoItem {
    fn default() -> Self {
        MockTodoItem::new(
            "2a".to_string(),
            "msg-test",
            Some(Prio::P1),
            None,
            Some(Tag("tag-test".to_string())),
        )
    }
}

pub struct MockListEnv {
    pub db: MockSqlDb,
}

impl MockListEnv {
    pub fn new() -> Result<Self> {
        let db = MockSqlDb::new()?;
        Ok(Self { db })
    }

    pub fn repo(&self) -> SqlTodoListRepository<'_> {
        SqlTodoListRepository::new(&self.db.conn)
    }
}

pub fn count_entries(conn: &Connection, table: &str) -> Result<i64> {
    let count: i64 = conn.query_row(&format!("SELECT COUNT(*) FROM {};", table), [], |row| {
        row.get(0)
    })?;
    Ok(count)
}

pub fn count_entries_where(condition: &str, conn: &Connection) -> Result<i64> {
    let count: i64 = conn.query_row(
        format!("SELECT COUNT(*) FROM todos WHERE {};", condition).as_ref(),
        [],
        |row| row.get(0),
    )?;
    Ok(count)
}
