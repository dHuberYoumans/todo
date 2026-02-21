use anyhow::Result;

use crate::domain::ListFilter;
use crate::domain::{Datetime, Metadata, Prio, Status, Tag, TodoItem};

// --------- TodoListRepository --------- //

// schema
pub trait TodoItemSchema {
    fn create_table(&self, name: Option<&str>) -> Result<()>;
}

// CRUD
pub trait TodoItemCreate {
    fn add(&self, item: &TodoItem) -> Result<()>;
}

pub trait TodoItemRead {
    fn fetch_item(&self, id: &str) -> Result<TodoItem>;
    fn fetch_list(&self, filter: Option<ListFilter>) -> Result<Vec<TodoItem>>;
}

pub trait TodoItemUpdate {
    fn update_task(&self, task: &str, id: &str) -> Result<()>;
    fn update(
        &self,
        due: Option<Datetime>,
        prio: Option<Prio>,
        satus: Option<Status>,
        tag: Option<Tag>,
        ids: Vec<String>,
    ) -> Result<()>;
    fn close_all(&self, prio: Option<Prio>) -> Result<()>;
}

pub trait TodoItemDelete {
    fn delete_item(&self, id: &str) -> Result<()>;
    fn delete_all_items(&self) -> Result<()>;
}

// Query
pub trait TodoItemQuery {
    fn fetch_by_due_date(
        &self,
        epoch_seconds: i64,
        filter: Option<ListFilter>,
    ) -> Result<Vec<TodoItem>>;
    fn fetch_by_prio(&self, prio: Prio) -> Result<Vec<TodoItem>>;
    fn fetch_by_tag(&self, tag: Tag, filter: Option<ListFilter>) -> Result<Vec<TodoItem>>;
    fn fetch_task_by_id(&self, id: &str) -> Result<Option<String>>;
}

pub trait TodoItemQueryColumns {
    fn fetch_tags(&self) -> Result<Vec<Tag>>;
    fn fetch_all_ids(&self) -> Result<Vec<String>>;
}

// Misc
pub trait TodoItemResolve {
    fn resolve_id(&self, prefix: &str) -> Result<String>;
}

pub trait TodoItemMetadata {
    fn fetch_item_and_metadata(&self, id: &str) -> Result<(TodoItem, Metadata)>;
}

pub trait TodoItemRepository:
    TodoItemSchema
    + TodoItemCreate
    + TodoItemRead
    + TodoItemUpdate
    + TodoItemDelete
    + TodoItemQuery
    + TodoItemQueryColumns
    + TodoItemResolve
    + TodoItemMetadata
{
}

impl<T> TodoItemRepository for T where
    T: TodoItemSchema
        + TodoItemCreate
        + TodoItemRead
        + TodoItemQuery
        + TodoItemQueryColumns
        + TodoItemUpdate
        + TodoItemDelete
        + TodoItemResolve
        + TodoItemMetadata
{
}

// --------- TodoListRepository --------- //

// CRUD
pub trait TodoListSchema {
    fn create_table(&self) -> Result<()>;
}

pub trait TodoListCreate {
    fn add(&self, list_name: &str) -> Result<()>;
}

pub trait TodoListRead {
    fn fetch_all(&self) -> Result<Vec<String>>;
    fn fetch_id(&self, list_name: &str) -> Result<i64>;
}

pub trait TodoListDelete {
    fn delete(&self, list_name: &str) -> Result<()>;
}

pub trait TodoListRepository:
    TodoListSchema + TodoListCreate + TodoListRead + TodoListDelete
{
}

impl<T> TodoListRepository for T where
    T: TodoListSchema + TodoListCreate + TodoListRead + TodoListDelete
{
}
