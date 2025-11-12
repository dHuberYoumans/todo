use anyhow::Result;

use crate::domain::{Status, Tag, TodoItem};

pub trait TodoItemRepository {
    fn create_table(&self) -> Result<()>;
    fn add(&self, item: &TodoItem) -> Result<()>;
    // fn fetch_by_status(&self, status: Status) -> Result<Vec<TodoItem>>;
    fn fetch_by_due_date(&self, epoch_seconds: i64) -> Result<Vec<TodoItem>>;
    // fn fetch_by_prio(&self, prio: Prio) -> Result<Vec<TodoItem>>;
    fn fetch_by_tag(&self, tag: Tag) -> Result<Vec<TodoItem>>;
    fn fetch_tags(&self) -> Result<Vec<Tag>>;
    fn fetch_all_ids(&self) -> Result<Vec<String>>;
    fn fetch_task_by_id(&self, id: &str) -> Result<Option<String>>;
    fn update_task(&self, task: &str, id: &str) -> Result<()>;
    fn update_status(&self, status: Status, id: &str) -> Result<()>;
    // fn update_tag(&self, id: String) -> Result<()>;
    // fn update_due(&self, id: String) -> Result<()>;
    // fn update_prio(&self, id: String) -> Result<()>;
    fn delete_task(&self, id: &str) -> Result<()>;
    fn resolve_id(&self, prefix: &str) -> Result<String>;
}

pub trait TodoListRepository {
    fn create_table(&self) -> Result<()>;
    fn add(&self, list_name: &str) -> Result<()>;
    fn fetch_all(&self) -> Result<Vec<String>>;
    fn fetch_id(&self, list_name: &str) -> Result<i64>;
    fn delete(&self, list_name: &str) -> Result<()>;
}
