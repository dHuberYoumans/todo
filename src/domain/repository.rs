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
    fn fetch_all_ids(&self) -> Result<Vec<i64>>;
    fn fetch_task_by_id(&self, id: i64) -> Result<Option<String>>;
    // TODO: write just 1 update function?
    fn update_task(&self, task: &str, id: i64) -> Result<()>;
    fn update_status(&self, status: Status, id: i64) -> Result<()>;
    // fn update_tag(&self, id: i64) -> Result<()>;
    // fn update_due(&self, id: i64) -> Result<()>;
    // fn update_prio(&self, id: i64) -> Result<()>;
    fn delete_task(&self, id: i64) -> Result<()>;
}

pub trait TodoListRepository {
    fn create_table(&self) -> Result<()>;
    fn add(&self, list_name: &str) -> Result<()>;
    fn fetch_all(&self) -> Result<Vec<String>>;
    fn fetch_id(&self, list_name: &str) -> Result<i64>;
    fn delete(&self, list_name: &str) -> Result<()>;
}
