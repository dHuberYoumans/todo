use anyhow::{Context, Result};

use crate::domain::{TodoItemDelete, TodoList};

impl TodoList {
    pub fn delete_item(&mut self, repo: &impl TodoItemDelete, id: &str) -> Result<()> {
        repo.delete_item(id)
            .context(format!("✘ Couldn't delete item {id}"))
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
        fn delete_item(&self, id: &str) -> Result<()> {
            let mut todos = self.todos.borrow_mut();
            todos.retain(|todo| todo.id == id);
            Ok(())
        }

        fn delete_all_items(&self) -> Result<()> {
            unreachable!()
        }
    }

    impl TodoItemDelete for FailingItemRepo {
        fn delete_item(&self, _: &str) -> Result<()> {
            bail!("Fake error while deleting item")
        }

        fn delete_all_items(&self) -> Result<()> {
            unreachable!()
        }
    }

    #[test]
    fn should_provide_context_upon_failure() {
        let repo = FailingItemRepo;
        let mut todo_list = TodoList::new();
        let err = todo_list.delete_item(&repo, "test-id-1");
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't delete item"));
    }

    #[test]
    fn should_delete_item_by_id() -> Result<()> {
        let repo = FakeItemRepo::new();
        let mut todo_list = TodoList::new();
        assert_eq!(repo.len(), 2);
        todo_list.delete_item(&repo, "test-id-1")?;
        assert_eq!(repo.len(), 1);
        Ok(())
    }
}
