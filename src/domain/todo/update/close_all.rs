use anyhow::{Context, Result};

use crate::domain::{Prio, TodoItemUpdate, TodoList};

impl TodoList {
    pub fn close_all(&self, repo: &impl TodoItemUpdate, prio: Option<Prio>) -> Result<()> {
        let err_context = match prio {
            Some(prio) => format!("✘ Couldn't close all todos with prio {}", prio),
            None => "✘ Couldn't close all todos".to_string(),
        };
        repo.close_all(prio).context(err_context)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    use crate::domain::{Datetime, Prio, Status, Tag, TodoItem, TodoItemUpdate};

    struct FakeItemRepo {
        todos: RefCell<Vec<TodoItem>>,
    }

    struct FailingRepo;

    impl FakeItemRepo {
        fn new() -> Self {
            let todo_1 = TodoItem {
                id: "todo-1".to_string(),
                task: "task-1".to_string(),
                due: Datetime::epoch(),
                status: Status::Open,
                prio: Prio::P1,
                tag: Tag::empty(),
            };
            let todo_2 = TodoItem {
                id: "todo-2".to_string(),
                task: "task-2".to_string(),
                due: Datetime::epoch(),
                status: Status::Open,
                prio: Prio::P1,
                tag: Tag::empty(),
            };
            let todo_3 = TodoItem {
                id: "todo-3".to_string(),
                task: "task-3".to_string(),
                due: Datetime::epoch(),
                status: Status::Open,
                prio: Prio::Empty,
                tag: Tag::empty(),
            };
            Self {
                todos: RefCell::new(vec![todo_1, todo_2, todo_3]),
            }
        }

        fn len(&self) -> usize {
            self.todos.borrow().len()
        }

        fn get_closed_todos(&self) -> Vec<TodoItem> {
            self.todos
                .borrow()
                .iter()
                .filter(|todo| todo.status == Status::Closed)
                .cloned()
                .collect()
        }
    }

    impl TodoItemUpdate for FailingRepo {
        fn update(
            &self,
            _: Option<Datetime>,
            _: Option<Prio>,
            _: Option<Status>,
            _: Option<Tag>,
            _: Vec<String>,
        ) -> Result<()> {
            unreachable!()
        }

        fn close_all(&self, _: Option<Prio>) -> Result<()> {
            bail!("Fake error while closing todos")
        }

        fn update_task(&self, _: &str, _: &str) -> Result<()> {
            unreachable!()
        }
    }

    impl TodoItemUpdate for FakeItemRepo {
        fn update(
            &self,
            _: Option<Datetime>,
            _: Option<Prio>,
            _: Option<Status>,
            _: Option<Tag>,
            _: Vec<String>,
        ) -> Result<()> {
            unreachable!()
        }

        fn close_all(&self, prio: Option<Prio>) -> Result<()> {
            let mut todos = self.todos.borrow_mut();
            for todo in todos.iter_mut() {
                if prio.is_none_or(|prio| todo.prio == prio) {
                    todo.status = Status::Closed;
                }
            }
            Ok(())
        }

        fn update_task(&self, _: &str, _: &str) -> Result<()> {
            unreachable!()
        }
    }

    #[test]
    fn should_provide_context_upon_failure_given_no_prio() {
        let repo = FailingRepo;
        let todo_list = TodoList::new();
        let err = todo_list.close_all(&repo, Some(Prio::P1));
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't close all todos with prio"));
    }

    #[test]
    fn should_provide_context_upon_failure_given_prio() {
        let repo = FailingRepo;
        let todo_list = TodoList::new();
        let err = todo_list.close_all(&repo, None);
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't close all todos"));
    }

    #[test]
    fn should_close_all_todos_if_prio_is_not_given() -> Result<()> {
        let repo = FakeItemRepo::new();
        assert_eq!(repo.len(), 3);
        let todo_list = TodoList::new();
        todo_list.close_all(&repo, None)?;
        assert_eq!(repo.get_closed_todos().len(), 3);
        Ok(())
    }

    #[test]
    fn should_close_all_todos_with_given_prio() -> Result<()> {
        let repo = FakeItemRepo::new();
        assert_eq!(repo.get_closed_todos().len(), 0);
        let todo_list = TodoList::new();
        todo_list.close_all(&repo, Some(Prio::P1))?;
        assert_eq!(repo.get_closed_todos().len(), 2);
        Ok(())
    }
}
