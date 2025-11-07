use anyhow::Result;

use crate::mock::*;
use todo::domain::TodoItemRepository;
use todo::domain::{Status, Tag};

// fn create_table() -> Result<()>{
// // fn fetch_by_status(&self, status: Status) -> Result<Vec<TodoItem>>;
// // fn fetch_by_prio(&self, prio: Prio) -> Result<Vec<TodoItem>>;
// // fn update_tag(&self, id: i64) -> Result<()>;
// // fn update_due(&self, id: i64) -> Result<()>;
// // fn update_prio(&self, id: i64) -> Result<()>;

/**************** TODO ITEM REPOSITORY *****************/

#[test]
fn add() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item = MockTodoItem::default();
    let repo = mock_env.repo("todos");

    repo.add(&mock_item.item)?;

    let count: i64 = count_entries(&mock_env.db.conn, "todos")?;
    assert_eq!(count, 1);

    Ok(())
}

#[test]
fn fetch_by_tag() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one =
        MockTodoItem::new(1, "test-msg-1", None, None, Some(Tag("test-tag-1".into())));
    let mock_item_two =
        MockTodoItem::new(2, "test-msg-2", None, None, Some(Tag("test-tag-2".into())));
    let repo = mock_env.repo("todos");

    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;
    let response = repo.fetch_by_tag(Tag("test-tag-2".into()))?;

    assert_eq!(response.len(), 1);

    Ok(())
}

#[test]
fn fetch_task_by_id() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one =
        MockTodoItem::new(1, "test-msg-1", None, None, Some(Tag("test-tag-1".into())));
    let mock_item_two =
        MockTodoItem::new(2, "test-msg-2", None, None, Some(Tag("test-tag-2".into())));
    let repo = mock_env.repo("todos");

    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;
    let response = repo.fetch_task_by_id(1)?;

    assert_eq!(response, Some("test-msg-1".to_string()));

    Ok(())
}

#[test]
fn update_task() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item = MockTodoItem::default();
    let repo = mock_env.repo("todos");
    repo.add(&mock_item.item)?;

    repo.update_task("updated", 1)?;
    let count = count_entries_where("task = 'updated'", &mock_env.db.conn)?;
    assert_eq!(count, 1);

    Ok(())
}

#[test]
fn update_status() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item = MockTodoItem::default();
    let repo = mock_env.repo("todos");
    repo.add(&mock_item.item)?;
    let count_open = count_entries_where("status = 1", &mock_env.db.conn)?;
    assert_eq!(count_open, 1);

    repo.update_status(Status::Closed, 1)?;
    let count_open = count_entries_where("status = 1", &mock_env.db.conn)?;
    assert_eq!(count_open, 0);

    Ok(())
}

#[test]
fn delete_task() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item = MockTodoItem::default();
    let repo = mock_env.repo("todos");
    repo.add(&mock_item.item)?;

    repo.delete_task(1)?;
    let count = count_entries(&mock_env.db.conn, "todos")?;
    assert_eq!(count, 0);

    Ok(())
}

#[test]
fn fetch_tags() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one =
        MockTodoItem::new(1, "test-msg-1", None, None, Some(Tag("test-tag-1".into())));
    let mock_item_two =
        MockTodoItem::new(2, "test-msg-2", None, None, Some(Tag("test-tag-2".into())));
    let repo = mock_env.repo("todos");
    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;

    let tags = repo.fetch_tags()?;
    assert_eq!(tags[0], Tag("test-tag-1".to_string()));
    assert_eq!(tags[1], Tag("test-tag-2".to_string()));

    Ok(())
}

#[test]
fn fetch_all_ids() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one =
        MockTodoItem::new(1, "test-msg-1", None, None, Some(Tag("test-tag-1".into())));
    let mock_item_two =
        MockTodoItem::new(2, "test-msg-2", None, None, Some(Tag("test-tag-2".into())));
    let repo = mock_env.repo("todos");
    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;

    let tags = repo.fetch_all_ids()?;
    assert_eq!(tags[0], 1);
    assert_eq!(tags[1], 2);

    Ok(())
}
