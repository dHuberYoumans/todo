use anyhow::{bail, Context, Result};

use crate::domain::{TodoItemUpdate, TodoList};

impl TodoList {
    pub fn update_task(&self, repo: &impl TodoItemUpdate, msg: &str, id: &str) -> Result<()> {
        if msg.is_empty() {
            bail!("✘ Can't have a todo with an empty task. Update aborted")
        };
        repo.update_task(msg, id)
            .context(format!("✘ Couldn't update the task with ID {}", id))?;
        Ok(())
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

    struct FailingRepo;

    impl TodoItemUpdate for FailingRepo {
        fn update(
            &self,
            _: Option<crate::domain::Datetime>,
            _: Option<crate::domain::Prio>,
            _: Option<crate::domain::Status>,
            _: Option<crate::domain::Tag>,
            _: Vec<String>,
        ) -> Result<()> {
            unreachable!()
        }

        fn close_all(&self, _: Option<Prio>) -> Result<()> {
            unreachable!()
        }

        fn update_task(&self, _: &str, _: &str) -> Result<()> {
            bail!("Fake error while updating the task")
        }
    }

    impl FakeItemRepo {
        fn new() -> Self {
            let todo = TodoItem {
                id: "todo-1".to_string(),
                task: "task-1".to_string(),
                due: Datetime::epoch(),
                status: Status::Open,
                prio: Prio::P1,
                tag: Tag::empty(),
            };

            Self {
                todos: RefCell::new(vec![todo]),
            }
        }

        fn get_task(&self) -> String {
            self.todos.borrow()[0].task.to_string()
        }
    }

    impl TodoItemUpdate for FakeItemRepo {
        fn update(
            &self,
            _: Option<crate::domain::Datetime>,
            _: Option<crate::domain::Prio>,
            _: Option<crate::domain::Status>,
            _: Option<crate::domain::Tag>,
            _: Vec<String>,
        ) -> Result<()> {
            unreachable!()
        }

        fn close_all(&self, _: Option<Prio>) -> Result<()> {
            unreachable!()
        }

        fn update_task(&self, msg: &str, id: &str) -> Result<()> {
            let mut todos_iter = self.todos.borrow_mut();
            for todo in todos_iter.iter_mut().filter(|todo| todo.id == id) {
                todo.task = msg.to_string();
            }
            Ok(())
        }
    }

    #[test]
    fn should_provide_context_upon_failure_for_empty_task() {
        let repo = FailingRepo;
        let todo_list = TodoList::new();
        let err = todo_list.update_task(&repo, "", "test-id");
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Can't have a todo with an empty task. Update aborted"));
    }

    #[test]
    fn should_provide_context_upon_failure_with_valid_id() {
        let repo = FailingRepo;
        let todo_list = TodoList::new();
        let err = todo_list.update_task(&repo, "Bonsoir, Elliot", "test-id");
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't update the task with ID"));
    }

    #[test]
    fn should_update_task() -> Result<()> {
        let repo = FakeItemRepo::new();
        assert_eq!(repo.get_task(), "task-1");
        let todo_list = TodoList::new();
        let msg = "A New Hope";
        todo_list.update_task(&repo, msg, "todo-1")?;
        assert_eq!(repo.get_task(), "A New Hope");
        Ok(())
    }
}
