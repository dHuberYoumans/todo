use anyhow::{anyhow, Result};
use rusqlite::{named_params, Connection, OptionalExtension, ToSql};
use thiserror::Error;

use crate::domain::{
    Datetime, ListFilters, Metadata, Prio, Status, StatusFilter, Tag, TodoItem, TodoItemCreate,
    TodoItemDelete, TodoItemMetadata, TodoItemQuery, TodoItemQueryColumns, TodoItemRead,
    TodoItemResolve, TodoItemSchema, TodoItemUpdate, TodoListRead,
};
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

#[derive(Error, Debug)]
pub enum ItemNotFoundError {
    #[error("✘ No item with id='{0}' found")]
    InvalidId(String),
}

impl TodoItemSchema for SqlTodoItemRepository<'_> {
    fn create_table(&self, name: Option<&str>) -> Result<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {table} (
id TEXT PRIMARY KEY UNIQUE,
list_id INTEGER NOT NULL 
    REFERENCES {collection}(id)
    ON DELETE CASCADE,
task TEXT NOT NULL,
status INTEGER DEFAULT 0,
prio INTEGER,
due INTEGER,
tag TEXT,
created_at INTEGER,
last_updated INTEGER
);",
            table = name.unwrap_or(&self.name),
            collection = SqlTodoListRepository::TABLE
        );
        log::debug!("executing query `{}`", &sql);
        let _ = self.conn.execute(&sql, [])?;
        Ok(())
    }
}

impl TodoItemCreate for SqlTodoItemRepository<'_> {
    fn add(&self, item: &TodoItem) -> Result<()> {
        let sql = format!(
            "INSERT INTO {} (id, task, list_id, status, prio, due, tag, created_at, last_updated)
VALUES (:id, :task, :list_id, :status, :prio, :due, :tag, :created_at, :last_updated);",
            self.name
        );
        let list_id = self.collection.fetch_id(&self.name)?;
        log::debug!("executing query `{}`", &sql);
        let _ = self.conn.execute(
            &sql,
            named_params! {
                ":id": item.id,
                ":task": item.task,
                ":list_id": list_id,
                ":status": item.status,
                ":prio": item.prio,
                ":due": item.due,
                ":tag": item.tag,
                ":created_at": Datetime::now(),
                ":last_updated": Datetime::now(),
            },
        )?;
        Ok(())
    }
}

impl TodoItemRead for SqlTodoItemRepository<'_> {
    fn fetch_item(&self, id: &str) -> Result<TodoItem> {
        let id = self.resolve_id(id)?;
        let sql = format!("SELECT * FROM {} WHERE id=:id;", self.name);
        log::debug!("executing query `{}`", &sql);
        let mut stmt = self.conn.prepare(&sql)?;
        let item = stmt.query_row(named_params! {":id": id}, |row| {
            let item = TodoItem {
                id: row.get::<_, String>("id")?,
                task: row.get::<_, String>("task")?,
                status: row.get::<_, Status>("status")?,
                prio: row.get::<_, Prio>("prio")?,
                due: row.get::<_, Datetime>("due")?,
                tag: row.get::<_, Tag>("tag")?,
            };
            Ok(item)
        })?;
        Ok(item)
    }

    fn fetch_list(&self, filters: ListFilters) -> Result<Vec<TodoItem>> {
        let mut sql = format!("SELECT * FROM {}", self.name);
        let mut query = NamedQuery {
            clause: String::new(),
            params: Vec::new(),
        };
        let mut conditions = Vec::new();
        if let Some(filter_query) = parse_filters(filters) {
            conditions.push(filter_query.clause);
            query.params.extend(filter_query.params);
        }
        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }
        let mut stmt = self.conn.prepare(&sql)?;
        let tasks = stmt
            .query_map(query.named_params().as_slice(), |row| {
                let item = TodoItem {
                    id: row.get::<_, String>("id")?,
                    task: row.get::<_, String>("task")?,
                    status: row.get::<_, Status>("status")?,
                    prio: row.get::<_, Prio>("prio")?,
                    due: row.get::<_, Datetime>("due")?,
                    tag: row.get::<_, Tag>("tag")?,
                };
                Ok(item)
            })?
            .collect::<rusqlite::Result<Vec<TodoItem>>>()?;
        Ok(tasks)
    }
}

impl TodoItemUpdate for SqlTodoItemRepository<'_> {
    fn update(
        &self,
        due: Option<Datetime>,
        prio: Option<Prio>,
        status: Option<Status>,
        tag: Option<Tag>,
        ids: Vec<String>,
    ) -> Result<()> {
        let ids: Vec<String> = ids
            .iter()
            .map(|id| self.resolve_id(id))
            .collect::<Result<Vec<String>>>()?;
        let mut sets = Vec::new();
        let mut id_placeholders: Vec<String> = Vec::new();
        let mut params: Vec<(&str, &dyn ToSql)> = Vec::new();
        let mut id_keys: Vec<String> = Vec::with_capacity(ids.len());
        let now = Datetime::now();
        sets.push("last_updated=:last_updated".to_string());
        params.push((":last_updated", &now as &dyn ToSql));
        if due.is_some() {
            sets.push("due=:due".to_string());
            params.push((":due", &due as &dyn ToSql));
        };
        if prio.is_some() {
            sets.push("prio=:prio".to_string());
            params.push((":prio", &prio as &dyn ToSql));
        };
        if status.is_some() {
            sets.push("status=:status".to_string());
            params.push((":status", &status as &dyn ToSql));
        };
        if tag.is_some() {
            sets.push("tag=:tag".to_string());
            params.push((":tag", &tag as &dyn ToSql));
        };
        for i in 0..ids.len() {
            let key = format!(":id{}", i);
            id_placeholders.push(key.clone());
            id_keys.push(key);
        }
        for (i, id) in ids.iter().enumerate() {
            params.push((id_keys[i].as_str(), id as &dyn ToSql));
        }
        let sql = format!(
            "UPDATE {} SET {} WHERE id IN ({});",
            self.name,
            sets.join(", "),
            id_placeholders.join(", ")
        );
        log::debug!("executing query `{}`", &sql);
        let _ = self.conn.execute(&sql, params.as_slice())?;
        Ok(())
    }
    fn update_task(&self, task: &str, id: &str) -> Result<()> {
        let id = self.resolve_id(id)?;
        let sql = format!(
            " UPDATE {} SET task=:task, last_updated=:last_updated WHERE id=:id;",
            self.name
        );
        log::debug!("executing query `{}`", &sql);
        let _ = self.conn.execute(
            &sql,
            named_params! { ":task": task, ":last_updated": Datetime::now(), ":id": id},
        )?;
        Ok(())
    }

    fn close_all(&self, prio: Option<Prio>) -> Result<()> {
        let tasks = if let Some(prio) = prio {
            self.fetch_by_prio(prio)?
        } else {
            self.fetch_list(ListFilters::default())?
        };
        let ids: Vec<String> = tasks.iter().map(|item| item.id.clone()).collect();
        let sets = "last_updated=:last_updated, status=:status".to_string();
        let mut id_placeholders: Vec<String> = Vec::new();
        let mut params: Vec<(&str, &dyn ToSql)> = Vec::new();
        let mut id_keys: Vec<String> = Vec::with_capacity(ids.len());
        let now = Datetime::now();
        params.push((":last_updated", &now));
        params.push((":status", &Status::Closed));
        for i in 0..ids.len() {
            let key = format!(":id{}", i);
            id_placeholders.push(key.clone());
            id_keys.push(key);
        }
        for (i, id) in ids.iter().enumerate() {
            params.push((id_keys[i].as_str(), id as &dyn ToSql));
        }

        let sql = format!(
            "UPDATE {} SET {} WHERE id IN ({});",
            self.name,
            sets,
            id_placeholders.join(", ")
        );
        log::debug!("executing query `{}`", &sql);
        let _ = self.conn.execute(&sql, params.as_slice())?;
        Ok(())
    }
}

impl TodoItemDelete for SqlTodoItemRepository<'_> {
    fn delete_item(&self, id: &str) -> Result<()> {
        let id = self.resolve_id(id)?;
        let sql = format!("DELETE FROM {} WHERE id=:id;", self.name);
        log::debug!("executing query `{}`", &sql);
        let _ = self.conn.execute(&sql, named_params! {":id": id})?;
        Ok(())
    }

    fn delete_all_items(&self) -> Result<()> {
        let sql = format!("DELETE FROM {}", self.name);
        log::debug!("executing query `{}`", &sql);
        self.conn.execute(&sql, [])?;
        Ok(())
    }
}

impl TodoItemQuery for SqlTodoItemRepository<'_> {
    fn fetch_task_by_id(&self, id: &str) -> Result<Option<String>> {
        let id = self.resolve_id(id)?;
        let sql = format!("SELECT task FROM {} WHERE id=:id;", self.name);
        let result = self
            .conn
            .query_row(&sql, named_params! {":id": id}, |row| {
                row.get::<_, String>(0)
            })
            .optional()?;
        Ok(result)
    }

    fn fetch_by_prio(&self, prio: Prio) -> Result<Vec<TodoItem>> {
        let sql: String = format!("SELECT * FROM {} WHERE prio=:prio;", self.name);
        let mut stmt = self.conn.prepare(&sql)?;
        let entries = stmt.query_map(named_params! {":prio": prio}, |row| {
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

    fn fetch_by_due_date(&self, epoch_seconds: i64, filters: ListFilters) -> Result<Vec<TodoItem>> {
        let mut sql = format!("SELECT * FROM {}", self.name);
        let mut query = NamedQuery {
            clause: String::new(),
            params: vec![(":date".into(), Box::new(epoch_seconds))],
        };
        let mut conditions = vec!["due = :date".to_string()];
        if let Some(filter_query) = parse_filters(filters) {
            conditions.push(filter_query.clause);
            query.params.extend(filter_query.params);
        };
        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }
        let mut stmt = self.conn.prepare(&sql)?;
        let entries = stmt.query_map(query.named_params().as_slice(), |row| {
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

    fn fetch_by_tag(&self, tag: Tag, filters: ListFilters) -> Result<Vec<TodoItem>> {
        let mut sql = format!("SELECT * FROM {}", self.name);
        let mut query = NamedQuery {
            clause: String::new(),
            params: vec![(":tag".into(), Box::new(tag))],
        };
        let mut conditions = vec!["tag = :tag".to_string()];
        if let Some(filter_query) = parse_filters(filters) {
            conditions.push(filter_query.clause);
            query.params.extend(filter_query.params);
        };
        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }
        let mut stmt = self.conn.prepare(&sql)?;
        let entries = stmt.query_map(query.named_params().as_slice(), |row| {
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
}

impl TodoItemQueryColumns for SqlTodoItemRepository<'_> {
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

    fn fetch_all_ids(&self) -> Result<Vec<String>> {
        let sql = format!("SELECT id FROM {};", self.name);
        log::debug!("executing query `{}`", &sql);
        let mut stmt = self.conn.prepare(&sql)?;
        let ids = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ids)
    }
}

impl TodoItemMetadata for SqlTodoItemRepository<'_> {
    fn fetch_item_and_metadata(&self, id: &str) -> Result<(TodoItem, Metadata)> {
        let id = self.resolve_id(id)?;
        let sql = format!("SELECT * FROM {} WHERE id=:id;", self.name);
        let mut stmt = self.conn.prepare(&sql)?;
        let (item, metadata) = stmt.query_row(named_params! {":id": id}, |row| {
            let item = TodoItem {
                id: row.get::<_, String>("id")?,
                task: row.get::<_, String>("task")?,
                status: row.get::<_, Status>("status")?,
                prio: row.get::<_, Prio>("prio")?,
                due: row.get::<_, Datetime>("due")?,
                tag: row.get::<_, Tag>("tag")?,
            };
            let metadata = Metadata {
                created_at: row.get::<_, Datetime>("created_at")?,
                last_updated: row.get::<_, Datetime>("last_updated")?,
            };
            Ok((item, metadata))
        })?;
        Ok((item, metadata))
    }
}

impl TodoItemResolve for SqlTodoItemRepository<'_> {
    fn resolve_id(&self, id: &str) -> Result<String> {
        let sql = format!("SELECT id FROM {} WHERE id LIKE :id || '%'", self.name);
        let mut stmt = self.conn.prepare(&sql)?;
        let ids = stmt
            .query_map(named_params! {":id": id}, |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;
        match ids.len() {
            0 => {
                let err = Err(ItemNotFoundError::InvalidId(id.into()).into());
                log::debug!("{:#?}", err);
                err
            }
            1 => {
                let id = ids[0].clone();
                Ok(id)
            }
            _ => Err(anyhow!("✘ Ambiguous prefix")),
        }
    }
}

pub struct NamedQuery {
    pub clause: String,
    pub params: Vec<(String, Box<dyn ToSql>)>,
}

impl NamedQuery {
    pub fn named_params(&self) -> Vec<(&str, &dyn ToSql)> {
        self.params
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_ref()))
            .collect()
    }
}

pub struct QueryConditions {
    pub clause: String,
    pub params: Vec<(String, Box<dyn ToSql>)>,
}

fn parse_filters(filters: ListFilters) -> Option<NamedQuery> {
    let mut builder = ConditionsBuilder::new();
    if let Some(status) = filters.status {
        match status {
            StatusFilter::All => {}
            StatusFilter::Done => builder.add("status", 0),
            StatusFilter::Do => builder.add("status", 1),
        }
    };
    if let Some(prio) = filters.prio {
        builder.add("prio", prio);
    }
    builder.build()
}

struct ConditionsBuilder {
    conditions: Vec<String>,
    params: Vec<(String, Box<dyn ToSql>)>,
}

impl ConditionsBuilder {
    fn new() -> Self {
        Self {
            conditions: Vec::new(),
            params: Vec::new(),
        }
    }

    fn add<T: ToSql + 'static>(&mut self, column: &str, value: T) {
        let param_name = format!(":{}", column);
        self.conditions.push(format!("{column} = {param_name}"));
        self.params.push((param_name, Box::new(value)));
    }

    fn build(self) -> Option<NamedQuery> {
        if self.conditions.is_empty() {
            None
        } else {
            Some(NamedQuery {
                clause: self.conditions.join(" AND "),
                params: self.params,
            })
        }
    }
}
