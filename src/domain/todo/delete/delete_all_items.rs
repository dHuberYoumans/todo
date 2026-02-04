use anyhow::{Context, Result};

use crate::domain::{TodoItemDelete, TodoList};

impl TodoList {
    pub fn delete_all_items(&self, repo: &impl TodoItemDelete) -> Result<()> {
        repo.delete_all_items()
            .context("✘ Couldn't delete all items")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    use crate::domain::{Datetime, Prio, Status, Tag, TodoItem};

    struct FakeItemRepo {
        todos: RefCell<Vec<TodoItem>>,
    }

    impl FakeItemRepo {
        fn new() -> Self {
            let todo_one = TodoItem {
                id: "test-id-1".to_string(),
                task: "some-task-1".to_string(),
                status: Status::Open,
                due: Datetime::epoch(),
                tag: Tag("some-tag-1".to_string()),
                prio: Prio::P3,
            };
            let todo_two = TodoItem {
                id: "test-id-2".to_string(),
                task: "some-task-2".to_string(),
                status: Status::Open,
                due: Datetime::epoch(),
                tag: Tag("some-tag-2".to_string()),
                prio: Prio::P3,
            };
            Self {
                todos: RefCell::new(vec![todo_one, todo_two]),
            }
        }

        fn len(&self) -> usize {
            self.todos.borrow().len()
        }
    }

    struct FailingItemRepo;

    impl TodoItemDelete for FakeItemRepo {
        fn delete_item(&self, _: &str) -> Result<()> {
            unreachable!()
        }

        fn delete_all_items(&self) -> Result<()> {
            let mut todos = self.todos.borrow_mut();
            *todos = Vec::new();
            Ok(())
        }
    }

    impl TodoItemDelete for FailingItemRepo {
        fn delete_item(&self, _: &str) -> Result<()> {
            unreachable!()
        }

        fn delete_all_items(&self) -> Result<()> {
            bail!("Fake error while deleting all items")
        }
    }

    #[test]
    fn should_provide_context_upon_failure() {
        let repo = FailingItemRepo;
        let todo_list = TodoList::new();
        let err = todo_list.delete_all_items(&repo);
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't delete all items"));
    }

    #[test]
    fn should_delete_all_todos() -> Result<()> {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        assert_eq!(repo.len(), 2);
        todo_list.delete_all_items(&repo)?;
        assert_eq!(repo.len(), 0);
        Ok(())
    }
}
