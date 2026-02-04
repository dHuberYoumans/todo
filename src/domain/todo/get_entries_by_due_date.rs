use anyhow::{Context, Result};

use crate::domain::{ListFilter, TodoItem, TodoItemQuery, TodoList};

impl TodoList {
    pub fn get_entries_by_due_date(
        &self,
        repo: &impl TodoItemQuery,
        epoch_seconds: i64,
        filter: Option<ListFilter>,
    ) -> Result<Vec<TodoItem>> {
        repo.fetch_by_due_date(epoch_seconds, filter)
            .context("✘ Couldn't fetch entries")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;
    use std::str::FromStr;

    use crate::domain::{Datetime, Prio, Status, Tag};

    struct FakeItemRepo {
        todos: RefCell<Vec<TodoItem>>,
    }

    impl FakeItemRepo {
        fn new(todos: Vec<TodoItem>) -> Self {
            Self {
                todos: RefCell::new(todos),
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

        fn fetch_task_by_id(&self, _: &str) -> Result<Option<String>> {
            unreachable!()
        }

        fn fetch_by_due_date(
            &self,
            epoch_seconds: i64,
            filter: Option<ListFilter>,
        ) -> Result<Vec<TodoItem>> {
            let todos_by_due_date: Vec<TodoItem> = self
                .todos
                .borrow()
                .iter()
                .filter(|todo| {
                    todo.due.timestamp == epoch_seconds
                        && match filter {
                            Some(ListFilter::None) => true,
                            Some(ListFilter::Done) => todo.status == Status::Closed,
                            Some(ListFilter::Do) => todo.status == Status::Open,
                            None => true,
                        }
                })
                .cloned()
                .collect::<Vec<TodoItem>>();
            Ok(todos_by_due_date)
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
            unreachable!()
        }

        fn fetch_by_due_date(&self, _: i64, _: Option<ListFilter>) -> Result<Vec<TodoItem>> {
            bail!("Fake error while fetching by due date")
        }
    }

    #[test]
    fn should_provide_context_upon_failing() {
        let repo = FailingItemRepo;
        let todo_list = TodoList::new();
        let err = todo_list.get_entries_by_due_date(&repo, 0, None);
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't fetch entries"));
    }

    #[test]
    fn should_fetch_all_todos_by_due_date_for_no_filter() {
        let todo_1 = TodoItem {
            id: "todo-1".to_string(),
            task: "task-1".to_string(),
            due: Datetime::from_str("13/06/2026").unwrap(),
            status: Status::Open,
            prio: Prio::Empty,
            tag: Tag::empty(),
        };
        let todo_2 = TodoItem {
            id: "todo-2".to_string(),
            task: "task-2".to_string(),
            due: Datetime::from_str("13/06/2026").unwrap(),
            status: Status::Closed,
            prio: Prio::Empty,
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
        let epoch_seconds = 1781301600;
        let repo = FakeItemRepo::new(vec![todo_1, todo_2, todo_3]);
        let todo_list = TodoList::new();
        let todos_by_due = todo_list
            .get_entries_by_due_date(&repo, epoch_seconds, None)
            .unwrap();
        assert_eq!(todos_by_due.len(), 2);
    }

    #[test]
    fn should_fetch_closed_todos_by_due_date_for_filter_done() {
        let todo_1 = TodoItem {
            id: "todo-1".to_string(),
            task: "task-1".to_string(),
            due: Datetime::from_str("13/06/2026").unwrap(),
            status: Status::Open,
            prio: Prio::Empty,
            tag: Tag::empty(),
        };
        let todo_2 = TodoItem {
            id: "todo-2".to_string(),
            task: "task-2".to_string(),
            due: Datetime::from_str("13/06/2026").unwrap(),
            status: Status::Closed,
            prio: Prio::Empty,
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
        let epoch_seconds = 1781301600;
        let repo = FakeItemRepo::new(vec![todo_1, todo_2, todo_3]);
        let todo_list = TodoList::new();
        let todos_by_due = todo_list
            .get_entries_by_due_date(&repo, epoch_seconds, Some(ListFilter::Done))
            .unwrap();
        assert_eq!(todos_by_due.len(), 1);
    }

    #[test]
    fn should_fetch_open_todos_by_due_date_for_filter_do() {
        let todo_1 = TodoItem {
            id: "todo-1".to_string(),
            task: "task-1".to_string(),
            due: Datetime::from_str("13/06/2026").unwrap(),
            status: Status::Open,
            prio: Prio::Empty,
            tag: Tag::empty(),
        };
        let todo_2 = TodoItem {
            id: "todo-2".to_string(),
            task: "task-2".to_string(),
            due: Datetime::from_str("13/06/2026").unwrap(),
            status: Status::Closed,
            prio: Prio::Empty,
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
        let epoch_seconds = 1781301600;
        let repo = FakeItemRepo::new(vec![todo_1, todo_2, todo_3]);
        let todo_list = TodoList::new();
        let todos_by_due = todo_list
            .get_entries_by_due_date(&repo, epoch_seconds, Some(ListFilter::Do))
            .unwrap();
        assert_eq!(todos_by_due.len(), 1);
    }
}
