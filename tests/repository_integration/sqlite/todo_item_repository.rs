use std::str::FromStr;

use anyhow::Result;

use crate::mock::*;
use todo::domain::{Datetime, ListFilters, Prio, Status, StatusFilter, Tag};
use todo::domain::{
    TodoItemCreate, TodoItemDelete, TodoItemMetadata, TodoItemQuery, TodoItemQueryColumns,
    TodoItemRead, TodoItemResolve, TodoItemUpdate,
};

/**************** TODO ITEM REPOSITORY *****************/

#[test]
fn add() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item = MockTodoItem::default();
    let repo = mock_env.repo("todos");

    repo.add(&mock_item.item)?;

    let count = count_entries(&mock_env.db.conn, "todos")?;
    assert_eq!(count, 1);

    Ok(())
}

#[test]
fn fetch_item() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item = MockTodoItem::default();
    let repo = mock_env.repo("todos");

    repo.add(&mock_item.item)?;
    let item = repo.fetch_item("2a")?;

    assert_eq!(item, mock_item.item);
    Ok(())
}

#[test]
fn fetch_list() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mut mock_item_one = MockTodoItem::new(
        "2a".to_string(),
        "test-msg-1",
        Some(Prio::P1),
        None,
        Some(Tag("test-tag-1".into())),
    );
    let mock_item_two = MockTodoItem::new(
        "39".to_string(),
        "test-msg-2",
        Some(Prio::P2),
        None,
        Some(Tag("test-tag-2".into())),
    );
    let repo = mock_env.repo("todos");

    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;

    // close task 1
    mock_item_one.item.status = Status::Closed;
    repo.update(
        None,
        None,
        Some(Status::Closed),
        None,
        vec!["2a".to_string()],
    )?;
    let items = repo.fetch_list(ListFilters::default())?;
    assert_eq!(items.len(), 2);
    assert_eq!(items[0], mock_item_one.item);
    assert_eq!(items[1], mock_item_two.item);

    let items = repo.fetch_list(ListFilters {
        status: Some(StatusFilter::Done),
        prio: None,
        due: None,
        tag: None,
    })?;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0], mock_item_one.item);

    let items = repo.fetch_list(ListFilters {
        status: Some(StatusFilter::Do),
        prio: None,
        due: None,
        tag: None,
    })?;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0], mock_item_two.item);

    let items = repo.fetch_list(ListFilters {
        status: None,
        prio: Some(Prio::P1),
        due: None,
        tag: None,
    })?;
    assert_eq!(items.len(), 1);
    assert_eq!(items[0], mock_item_one.item);

    Ok(())
}

#[test]
fn fetch_by_prio() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one =
        MockTodoItem::new("2a".to_string(), "test-msg-1", Some(Prio::P1), None, None);
    let mock_item_two =
        MockTodoItem::new("39".to_string(), "test-msg-2", Some(Prio::P2), None, None);
    let repo = mock_env.repo("todos");

    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;
    let response = repo.fetch_by_prio(Prio::P1)?;

    assert_eq!(response.len(), 1);

    Ok(())
}

#[test]
fn fetch_by_tag() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one = MockTodoItem::new(
        "2a".to_string(),
        "test-msg-1",
        Some(Prio::P1),
        None,
        Some(Tag("test-tag".into())),
    );
    let mock_item_two = MockTodoItem::new(
        "39".to_string(),
        "test-msg-2",
        None,
        None,
        Some(Tag("different-tag".into())),
    );
    let mock_item_three = MockTodoItem::new(
        "og".to_string(),
        "test-msg-3",
        Some(Prio::P3),
        None,
        Some(Tag("test-tag".into())),
    );

    let repo = mock_env.repo("todos");

    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;
    repo.add(&mock_item_three.item)?;
    let response = repo.fetch_by_tag(Tag("test-tag".into()), ListFilters::default())?;
    assert_eq!(response.len(), 2);
    let response = repo.fetch_by_tag(
        Tag("test-tag".into()),
        ListFilters {
            status: None,
            prio: Some(Prio::P1),
            due: None,
            tag: None,
        },
    )?;
    assert_eq!(response.len(), 1);
    Ok(())
}

#[test]
fn fetch_task_by_id() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one = MockTodoItem::new(
        "2a".to_string(),
        "test-msg-1",
        None,
        None,
        Some(Tag("test-tag-1".into())),
    );
    let mock_item_two = MockTodoItem::new(
        "39".to_string(),
        "test-msg-2",
        None,
        None,
        Some(Tag("test-tag-2".into())),
    );
    let repo = mock_env.repo("todos");

    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;
    let response = repo.fetch_task_by_id("2a")?;

    assert_eq!(response, Some("test-msg-1".to_string()));

    Ok(())
}

#[test]
fn update_task() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item = MockTodoItem::default();
    let repo = mock_env.repo("todos");
    repo.add(&mock_item.item)?;

    repo.update_task("updated", "2a")?;
    let count = count_entries_where("id = '2a'", &mock_env.db.conn)?;
    assert_eq!(count, 1);

    Ok(())
}

#[test]
fn update_single() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item = MockTodoItem::default();
    let repo = mock_env.repo("todos");
    repo.add(&mock_item.item)?;
    let initial_conditions = update_conditions(None, None, None, None);
    let count_initial: i64 = count_entries_where(&initial_conditions, &mock_env.db.conn)?;
    assert_eq!(count_initial, 1);

    // update due
    let new_due = Some(Datetime::from_str("01-09-2023")?);
    repo.update(new_due, None, None, None, vec!["2a".to_string()])?;
    let new_conditions = update_conditions(new_due, None, None, None);
    let count = count_entries_where(&new_conditions, &mock_env.db.conn)?;
    assert_eq!(count, 1);

    // update prio
    let new_prio = Some(Prio::P2);
    repo.update(None, new_prio, None, None, vec!["2a".to_string()])?;
    let new_conditions = update_conditions(new_due, new_prio, None, None);
    let count = count_entries_where(&new_conditions, &mock_env.db.conn)?;
    assert_eq!(count, 1);

    // update status
    let new_status = Some(Status::Closed);
    repo.update(None, None, new_status, None, vec!["2a".to_string()])?;
    let new_conditions = update_conditions(new_due, new_prio, new_status, None);
    let count = count_entries_where(&new_conditions, &mock_env.db.conn)?;
    assert_eq!(count, 1);

    // update tag
    let new_tag = Some(Tag("new tag".to_string()));
    repo.update(None, None, None, new_tag.clone(), vec!["2a".to_string()])?;
    let new_conditions = update_conditions(new_due, new_prio, new_status, new_tag.clone());
    dbg!(&new_conditions);
    let count = count_entries_where(&new_conditions, &mock_env.db.conn)?;
    assert_eq!(count, 1);

    Ok(())
}

#[test]
fn last_updated() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item = MockTodoItem::default();
    let repo = mock_env.repo("todos");
    repo.add(&mock_item.item)?;
    let (_item, metadata) = repo.fetch_item_and_metadata("2a")?;
    dbg!("{}", metadata.created_at);
    dbg!("{}", metadata.last_updated);

    let count = count_entries_where(
        &format!("last_updated = {}", metadata.last_updated.timestamp),
        &mock_env.db.conn,
    )?;
    assert_eq!(count, 1);

    Ok(())
}

#[test]
fn close_all() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one =
        MockTodoItem::new("2a".to_string(), "test-msg-1", Some(Prio::P1), None, None);
    let mock_item_two =
        MockTodoItem::new("39".to_string(), "test-msg-2", Some(Prio::P2), None, None);
    let repo = mock_env.repo("todos");
    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;
    let count_initial = count_entries_where("status = 1", &mock_env.db.conn)?;
    assert_eq!(count_initial, 2);

    repo.close_all(None)?;
    let count = count_entries_where("status = 1", &mock_env.db.conn)?;
    assert_eq!(count, 0);

    repo.update(
        None,
        None,
        Some(Status::Open),
        None,
        vec!["2a".to_string(), "39".to_string()],
    )?;

    repo.close_all(Some(Prio::P1))?;
    let count = count_entries_where("status = 1", &mock_env.db.conn)?;
    assert_eq!(count, 1);

    Ok(())
}

#[test]
fn update_batch() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one = MockTodoItem::new(
        "2a".to_string(),
        "test-msg-1",
        None,
        None,
        Some(Tag("test-tag-1".into())),
    );
    let mock_item_two = MockTodoItem::new(
        "39".to_string(),
        "test-msg-2",
        None,
        None,
        Some(Tag("test-tag-2".into())),
    );
    let repo = mock_env.repo("todos");
    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;
    let count_initial = count_entries_where("status = 1", &mock_env.db.conn)?;
    assert_eq!(count_initial, 2);

    repo.update(
        None,
        None,
        Some(Status::Closed),
        None,
        vec!["2a".to_string(), "39".to_string()],
    )?;
    let count = count_entries_where("status = 1", &mock_env.db.conn)?;
    assert_eq!(count, 0);

    repo.update(None, None, Some(Status::Open), None, vec!["2a".to_string()])?;
    let count = count_entries_where("status = 1", &mock_env.db.conn)?;
    assert_eq!(count, 1);

    repo.update(
        None,
        None,
        None,
        Some(Tag("newtag".to_string())),
        vec!["2a".to_string(), "39".to_string()],
    )?;
    let count = count_entries_where("tag = 'newtag'", &mock_env.db.conn)?;
    assert_eq!(count, 2);

    repo.update(None, Some(Prio::P3), None, None, vec!["39".to_string()])?;
    let count = count_entries_where("prio = 3", &mock_env.db.conn)?;
    assert_eq!(count, 1);

    Ok(())
}

fn update_conditions(
    due: Option<Datetime>,
    prio: Option<Prio>,
    status: Option<Status>,
    tag: Option<Tag>,
) -> String {
    let mut conditions = Vec::new();
    if let Some(due) = due {
        conditions.push(format!("due = {}", due.timestamp))
    } else {
        conditions.push(format!("due = {}", Datetime::default().timestamp))
    };
    match prio {
        Some(Prio::P1) => conditions.push("prio = 1".to_string()),
        Some(Prio::P2) => conditions.push("prio = 2".to_string()),
        Some(Prio::P3) => conditions.push("prio = 3".to_string()),
        _ => conditions.push("prio = 1".to_string()),
    };
    match status {
        Some(Status::Open) => conditions.push("status = 1".to_string()),
        Some(Status::Closed) => conditions.push("status = 0".to_string()),
        _ => conditions.push("status = 1".to_string()),
    };
    if let Some(tag) = tag {
        conditions.push(format!("tag = '{}'", tag.0));
    } else {
        conditions.push(format!("tag = {}", "'tag-test'"));
    }
    conditions.join(" AND ")
}

#[test]
fn delete_item() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item = MockTodoItem::default();
    let repo = mock_env.repo("todos");
    repo.add(&mock_item.item)?;

    repo.delete_item("2a")?;
    let count = count_entries(&mock_env.db.conn, "todos")?;
    assert_eq!(count, 0);

    Ok(())
}

#[test]
fn delte_all_items() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one = MockTodoItem::new(
        "2a".to_string(),
        "test-msg-1",
        None,
        None,
        Some(Tag("test-tag-1".into())),
    );
    let mock_item_two = MockTodoItem::new(
        "39".to_string(),
        "test-msg-2",
        None,
        None,
        Some(Tag("test-tag-2".into())),
    );
    let repo = mock_env.repo("todos");
    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;

    repo.delete_all_items()?;
    let count = count_entries(&mock_env.db.conn, "todos")?;
    assert_eq!(count, 0);

    Ok(())
}

#[test]
fn fetch_tags() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let mock_item_one = MockTodoItem::new(
        "2a".to_string(),
        "test-msg-1",
        None,
        None,
        Some(Tag("test-tag-1".into())),
    );
    let mock_item_two = MockTodoItem::new(
        "39".to_string(),
        "test-msg-2",
        None,
        None,
        Some(Tag("test-tag-2".into())),
    );
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
    let mock_item_one = MockTodoItem::new(
        "2a".to_string(),
        "test-msg-1",
        None,
        None,
        Some(Tag("test-tag-1".into())),
    );
    let mock_item_two = MockTodoItem::new(
        "39".to_string(),
        "test-msg-2",
        None,
        None,
        Some(Tag("test-tag-2".into())),
    );
    let repo = mock_env.repo("todos");
    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;

    let tags = repo.fetch_all_ids()?;
    assert_eq!(tags[0], "2a");
    assert_eq!(tags[1], "39");

    Ok(())
}

#[test]
fn resolve_unique_id() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let target_id = "1634f2b6747bafd9".to_string();
    let mock_item_one = MockTodoItem::new(target_id.clone(), "test-msg-1", None, None, None);
    let mock_item_two = MockTodoItem::new(
        "5d60d6f6f2d88d12".to_string(),
        "test-msg-1",
        None,
        None,
        None,
    );
    let repo = mock_env.repo("todos");
    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;

    let id = repo.resolve_id("1634")?;
    assert_eq!(id, target_id);

    Ok(())
}

#[test]
fn resolve_ambiguous_id() -> Result<()> {
    let mock_env = MockItemEnv::new()?;
    let target_id = "1634f2b6747bafd9".to_string();
    let mock_item_one = MockTodoItem::new(target_id.clone(), "test-msg-1", None, None, None);
    let mock_item_two = MockTodoItem::new(
        "1634f2a6747bafd9".to_string(),
        "test-msg-1",
        None,
        None,
        None,
    );
    let repo = mock_env.repo("todos");
    repo.add(&mock_item_one.item)?;
    repo.add(&mock_item_two.item)?;

    let result = repo.resolve_id("1634");
    assert!(result.is_err());

    Ok(())
}
