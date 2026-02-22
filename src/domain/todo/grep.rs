use anyhow::{Context, Result};

use crate::domain::{ListFilters, StatusFilter, TodoItem, TodoItemRead, TodoList};

#[derive(Clone, Debug)]
pub struct GrepOptions {
    pub case_insensitive: bool,
}

impl TodoList {
    pub fn grep(
        &self,
        repo: &impl TodoItemRead,
        pattern: &str,
        options: GrepOptions,
    ) -> Result<Vec<TodoItem>> {
        let mut todos = repo
            .fetch_list(ListFilters {
                status: Some(StatusFilter::All),
                prio: None,
            })
            .context("✘ Couldn't fetch todos while searching for pattern '{pattern}'")?;
        if options.case_insensitive {
            let pattern = pattern.to_string().to_lowercase();
            todos.retain(|todo| todo.task.to_lowercase().contains(&pattern))
        } else {
            todos.retain(|todo| todo.task.contains(pattern))
        };
        Ok(todos)
    }
}

pub fn search_in_task(pattern: &str, item: &TodoItem) -> bool {
    item.task.contains(pattern)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    use crate::domain::{Datetime, Prio, Status, Tag};

    struct FakeItemRepo {
        todos: RefCell<Vec<TodoItem>>,
    }

    struct FailingItemRepo;

    impl FakeItemRepo {
        fn new() -> Self {
            let long_task = "Title: This is a long task\n\nOnce upon a time there was a long long task to parse!".to_string();
            let short_task = "Title: This is a short task".to_string();
            let todo_short = TodoItem {
                id: "id-short".to_string(),
                task: short_task,
                due: Datetime::epoch(),
                tag: Tag("some-tag".to_string()),
                prio: Prio::Empty,
                status: Status::Open,
            };
            let todo_long = TodoItem {
                id: "id-long".to_string(),
                task: long_task,
                due: Datetime::epoch(),
                tag: Tag("some-tag".to_string()),
                prio: Prio::Empty,
                status: Status::Open,
            };
            Self {
                todos: RefCell::new(vec![todo_short, todo_long]),
            }
        }
    }

    impl TodoItemRead for FakeItemRepo {
        fn fetch_item(&self, _: &str) -> Result<TodoItem> {
            unreachable!()
        }

        fn fetch_list(&self, _: ListFilters) -> Result<Vec<TodoItem>> {
            Ok(self.todos.borrow().clone())
        }
    }

    impl TodoItemRead for FailingItemRepo {
        fn fetch_item(&self, _: &str) -> Result<TodoItem> {
            unreachable!()
        }

        fn fetch_list(&self, _: ListFilters) -> Result<Vec<TodoItem>> {
            bail!("Fake error while trying to fetch list")
        }
    }

    #[test]
    fn should_provide_context_upon_failure() {
        let repo = FailingItemRepo;
        let todo_list = TodoList::new();
        let options = GrepOptions {
            case_insensitive: false,
        };
        let err = todo_list.grep(&repo, "any pattern", options);
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't fetch todos while searching for pattern"));
    }

    #[test]
    fn should_return_empty_vec_if_miss() -> Result<()> {
        let miss = "miss";
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let options = GrepOptions {
            case_insensitive: false,
        };
        let todos_miss = todo_list.grep(&repo, miss, options)?;
        assert!(todos_miss.is_empty());
        Ok(())
    }

    #[test]
    fn should_return_todos_containg_pattern() -> Result<()> {
        let pattern = "Title";
        let pattern_long = "long long";
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let options = GrepOptions {
            case_insensitive: false,
        };
        let todos_match = todo_list.grep(&repo, pattern_long, options.clone())?;
        assert_eq!(todos_match.len(), 1);
        let todos_match = todo_list.grep(&repo, pattern, options)?;
        assert_eq!(todos_match.len(), 2);
        Ok(())
    }
    #[test]
    fn should_return_todos_containg_pattern_case_insensitive() -> Result<()> {
        let pattern_long = "Long long";
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let options = GrepOptions {
            case_insensitive: true,
        };
        let todos_match = todo_list.grep(&repo, pattern_long, options)?;
        assert_eq!(todos_match.len(), 1);
        Ok(())
    }
}
