use anyhow::{Context, Result};

use crate::domain::{ListFilters, TodoItem, TodoItemRead, TodoList};

impl TodoList {
    pub fn get_list(
        &self,
        repo: &impl TodoItemRead,
        filters: ListFilters,
    ) -> Result<Vec<TodoItem>> {
        repo.fetch_list(filters).context("✘ Couldn't fetch todos")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    use crate::domain::{Datetime, Prio, Status, StatusFilter, Tag};

    struct FakeItemRepo {
        todos: RefCell<Vec<TodoItem>>,
    }

    struct FailingItemRepo;

    impl FakeItemRepo {
        fn new() -> Self {
            let todo_1 = TodoItem {
                id: "todo-open".to_string(),
                task: "task-1".to_string(),
                due: Datetime::epoch(),
                status: Status::Open,
                prio: Prio::Empty,
                tag: Tag::empty(),
            };
            let todo_2 = TodoItem {
                id: "todo-closed".to_string(),
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

    impl TodoItemRead for FailingItemRepo {
        fn fetch_item(&self, _: &str) -> Result<TodoItem> {
            unreachable!()
        }

        fn fetch_list(&self, _: ListFilters) -> Result<Vec<TodoItem>> {
            bail!("Fake error while fetching list")
        }
    }

    impl TodoItemRead for FakeItemRepo {
        fn fetch_list(&self, filters: ListFilters) -> Result<Vec<TodoItem>> {
            let todos: Vec<TodoItem> = self
                .todos
                .borrow()
                .iter()
                .filter(|todo| match filters.status {
                    None => true,
                    Some(StatusFilter::All) => true,
                    Some(StatusFilter::Do) => todo.status == Status::Open,
                    Some(StatusFilter::Done) => todo.status == Status::Closed,
                })
                .cloned()
                .collect();
            Ok(todos)
        }

        fn fetch_item(&self, _: &str) -> Result<TodoItem> {
            unreachable!()
        }
    }

    #[test]
    fn should_provide_context_upon_failing() {
        let repo = FailingItemRepo;
        let todo_list = TodoList::new();
        let err = todo_list.get_list(&repo, ListFilters::default());
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't fetch todos"));
    }

    #[test]
    fn should_fetch_all_todos_for_no_filter() {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let todos = todo_list.get_list(&repo, ListFilters::default()).unwrap();
        assert_eq!(todos.len(), 2)
    }

    #[test]
    fn should_fetch_open_todos_for_filter_do() {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let todos = todo_list
            .get_list(
                &repo,
                ListFilters {
                    status: Some(StatusFilter::Do),
                    prio: None,
                    due: None,
                    tag: None,
                },
            )
            .unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].id, "todo-open");
    }

    #[test]
    fn should_fetch_closed_todos_for_filter_done() {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let todos = todo_list
            .get_list(
                &repo,
                ListFilters {
                    status: Some(StatusFilter::Done),
                    prio: None,
                    due: None,
                    tag: None,
                },
            )
            .unwrap();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].id, "todo-closed");
    }
}
