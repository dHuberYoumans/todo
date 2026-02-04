use anyhow::{Context, Result};

use crate::domain::{TodoItemQuery, TodoList};

impl TodoList {
    pub fn get_entry(&mut self, repo: &impl TodoItemQuery, id: &str) -> Result<Option<String>> {
        let entry = repo
            .fetch_task_by_id(id)
            .context(format!("✘ Couldn't find entry with id {}", id))?;
        Ok(entry)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    use crate::domain::{Datetime, ListFilter, Prio, Status, Tag, TodoItem};

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

    impl TodoItemQuery for FakeItemRepo {
        fn fetch_by_tag(
            &self,
            _: crate::domain::Tag,
            _: Option<ListFilter>,
        ) -> Result<Vec<TodoItem>> {
            unreachable!()
        }

        fn fetch_by_prio(&self, _: crate::domain::Prio) -> Result<Vec<TodoItem>> {
            unreachable!()
        }

        fn fetch_task_by_id(&self, id: &str) -> Result<Option<String>> {
            let entry: Option<String> = self
                .todos
                .borrow()
                .iter()
                .filter(|todo| todo.id == id)
                .next()
                .map(|todo| todo.task.clone());
            Ok(entry)
        }

        fn fetch_by_due_date(&self, _: i64, _: Option<ListFilter>) -> Result<Vec<TodoItem>> {
            unreachable!()
        }
    }

    impl TodoItemQuery for FailingItemRepo {
        fn fetch_by_tag(
            &self,
            _: crate::domain::Tag,
            _: Option<ListFilter>,
        ) -> Result<Vec<TodoItem>> {
            unreachable!()
        }

        fn fetch_by_prio(&self, _: crate::domain::Prio) -> Result<Vec<TodoItem>> {
            unreachable!()
        }

        fn fetch_task_by_id(&self, _: &str) -> Result<Option<String>> {
            bail!("Fake error while fetching by id")
        }

        fn fetch_by_due_date(&self, _: i64, _: Option<ListFilter>) -> Result<Vec<TodoItem>> {
            unreachable!()
        }
    }

    #[test]
    fn should_provide_context_upon_failing() {
        let repo = FailingItemRepo;
        let mut todo_list = TodoList::new();
        let err = todo_list.get_entry(&repo, "unfindable-id");
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't find entry with id"))
    }

    #[test]
    fn should_fetch_task_from_id() {
        let repo = FakeItemRepo::new();
        let mut todo_list = TodoList::new();
        let todo = todo_list.get_entry(&repo, "todo-1").unwrap().unwrap();
        assert_eq!(todo, "task-1");
    }
}
