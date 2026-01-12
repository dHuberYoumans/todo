use anyhow::Result;

use crate::commands::ListFilter;
use crate::domain::{Datetime, Metadata, Prio, Status, Tag, TodoItem};

pub trait TodoItemRepository {
    fn create_table(&self) -> Result<()>;
    fn add(&self, item: &TodoItem) -> Result<()>;
    // fn fetch_by_status(&self, status: Status) -> Result<Vec<TodoItem>>;
    fn fetch_by_due_date(
        &self,
        epoch_seconds: i64,
        filter: Option<ListFilter>,
    ) -> Result<Vec<TodoItem>>;
    fn fetch_by_prio(&self, prio: Prio) -> Result<Vec<TodoItem>>;
    fn fetch_by_tag(&self, tag: Tag, filter: Option<ListFilter>) -> Result<Vec<TodoItem>>;
    fn fetch_tags(&self) -> Result<Vec<Tag>>;
    fn fetch_all_ids(&self) -> Result<Vec<String>>;
    fn fetch_task_by_id(&self, id: &str) -> Result<Option<String>>;
    fn fetch_item(&self, id: &str) -> Result<TodoItem>;
    fn fetch_item_and_metadata(&self, id: &str) -> Result<(TodoItem, Metadata)>;
    fn fetch_list(&self, filter: Option<ListFilter>) -> Result<Vec<TodoItem>>;
    fn update_task(&self, task: &str, id: &str) -> Result<()>;
    fn update(
        &self,
        due: Option<Datetime>,
        prio: Option<Prio>,
        satus: Option<Status>,
        tag: Option<Tag>,
        ids: Vec<String>,
    ) -> Result<()>;
    fn delete_task(&self, id: &str) -> Result<()>;
    fn close_all(&self, prio: Option<Prio>) -> Result<()>;
    fn resolve_id(&self, prefix: &str) -> Result<String>;
}

pub trait TodoListRepository {
    fn create_table(&self) -> Result<()>;
    fn add(&self, list_name: &str) -> Result<()>;
    fn fetch_all(&self) -> Result<Vec<String>>;
    fn fetch_id(&self, list_name: &str) -> Result<i64>;
    fn delete(&self, list_name: &str) -> Result<()>;
}
