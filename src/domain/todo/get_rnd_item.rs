use anyhow::{Context, Result};

use crate::domain::{ListFilters, Prio, StatusFilter, TodoItem, TodoItemRead, TodoList};

impl TodoList {
    pub fn get_rnd_item(&self, repo: &impl TodoItemRead) -> Result<Option<TodoItem>> {
        let todos = repo
            .fetch_list(ListFilters {
                status: Some(StatusFilter::Do),
                prio: None,
                due: None,
                tag: None,
            })
            .context("✘ Couldn't fetch todos while trying to retrieve a random todo")?;
        let rnd_todos: Vec<TodoItem> = todos
            .iter()
            .filter(|todo| todo.prio == Prio::RND)
            .cloned()
            .collect();
        if rnd_todos.is_empty() {
            return Ok(None);
        };
        let idx: usize = rand::random_range(0..rnd_todos.len());
        Ok(Some(rnd_todos[idx].clone()))
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

    struct FailingItemRepo;

    impl FakeItemRepo {
        fn new() -> Self {
            let todo_one = TodoItem {
                id: "test-id-1".to_string(),
                task: "some-task-1".to_string(),
                status: Status::Open,
                due: Datetime::epoch(),
                tag: Tag("some-tag-1".to_string()),
                prio: Prio::RND,
            };
            let todo_two = TodoItem {
                id: "test-id-2".to_string(),
                task: "some-task-2".to_string(),
                status: Status::Open,
                due: Datetime::epoch(),
                tag: Tag("some-tag-2".to_string()),
                prio: Prio::RND,
            };
            let todo_three = TodoItem {
                id: "test-id-3".to_string(),
                task: "some-task-3".to_string(),
                status: Status::Open,
                due: Datetime::epoch(),
                tag: Tag("some-tag-2".to_string()),
                prio: Prio::RND,
            };
            Self {
                todos: RefCell::new(vec![todo_one, todo_two, todo_three]),
            }
        }

        fn len(&self) -> usize {
            self.todos.borrow().len()
        }

        fn get_todos(&self) -> Vec<TodoItem> {
            self.todos.borrow().clone()
        }
    }

    impl TodoItemRead for FakeItemRepo {
        fn fetch_item(&self, _: &str) -> Result<TodoItem> {
            unreachable!()
        }

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
    }

    impl TodoItemRead for FailingItemRepo {
        fn fetch_list(&self, _: ListFilters) -> Result<Vec<TodoItem>> {
            bail!("Fake error while trying to get items")
        }

        fn fetch_item(&self, _: &str) -> Result<TodoItem> {
            unreachable!()
        }
    }

    #[test]
    fn should_return_a_rnd_item() -> Result<()> {
        let repo = FakeItemRepo::new();
        assert_eq!(repo.len(), 3);
        let todos = repo.get_todos();
        let todo_list = TodoList::new();
        let rnd_item: Option<TodoItem> = todo_list.get_rnd_item(&repo)?;
        assert!(rnd_item.is_some());
        assert!(todos.contains(&rnd_item.unwrap()));
        Ok(())
    }

    #[test]
    fn should_proide_context_upon_failure() {
        let repo = FailingItemRepo;
        let todo_list = TodoList::new();
        let err = todo_list.get_rnd_item(&repo);
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't fetch todos while trying to retrieve a random todo"));
    }
}
