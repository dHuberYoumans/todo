use anyhow::{Context, Result};

use crate::domain::{Metadata, TodoItem, TodoItemMetadata, TodoList};

impl TodoList {
    pub fn get_entry_with_metadata(
        &self,
        repo: &impl TodoItemMetadata,
        id: &str,
    ) -> Result<(TodoItem, Metadata)> {
        repo.fetch_item_and_metadata(id)
            .context("✘ Couldn't fetch entry")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::{cell::RefCell, str::FromStr};

    use crate::domain::{Datetime, Prio, Status, Tag, TodoItem};

    struct FakeItemRepo {
        todos: RefCell<Vec<TodoItem>>,
    }

    impl FakeItemRepo {
        fn new() -> Self {
            let todo_1 = TodoItem {
                id: "todo-1".to_string(),
                task: "task-1".to_string(),
                due: Datetime::epoch(),
                status: Status::Open,
                prio: Prio::Empty,
                tag: Tag::empty(),
            };
            let todo_2 = TodoItem {
                id: "todo-2".to_string(),
                task: "task-2".to_string(),
                due: Datetime::epoch(),
                status: Status::Closed,
                prio: Prio::Empty,
                tag: Tag::empty(),
            };
            Self {
                todos: RefCell::new(vec![todo_1, todo_2]),
            }
        }
    }

    struct FailingItemRepo;

    impl TodoItemMetadata for FakeItemRepo {
        fn fetch_item_and_metadata(&self, id: &str) -> Result<(TodoItem, Metadata)> {
            let todo_item = self
                .todos
                .borrow()
                .iter()
                .filter(|todo| todo.id == id)
                .next()
                .unwrap()
                .clone();
            let metadata = Metadata {
                created_at: Datetime::from_str("13/06/2025")?,
                last_updated: Datetime::from_str("13/06/2026")?,
            };
            Ok((todo_item, metadata))
        }
    }

    impl TodoItemMetadata for FailingItemRepo {
        fn fetch_item_and_metadata(&self, _: &str) -> Result<(TodoItem, Metadata)> {
            bail!("Fake error while fetching entry with metadata")
        }
    }

    #[test]
    fn should_provide_context_upon_failure() {
        let repo = FailingItemRepo;
        let todo_list = TodoList::new();
        let err = todo_list.get_entry_with_metadata(&repo, "unfindable-id");
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't fetch entry"));
    }

    #[test]
    fn should_fetch_item_with_metadata() -> Result<()> {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let (todo, metadata) = todo_list.get_entry_with_metadata(&repo, "todo-1")?;
        assert_eq!(todo.id, "todo-1");
        assert_eq!(metadata.created_at, Datetime::from_str("13/06/2025")?);
        assert_eq!(metadata.last_updated, Datetime::from_str("13/06/2026")?);
        Ok(())
    }
}
