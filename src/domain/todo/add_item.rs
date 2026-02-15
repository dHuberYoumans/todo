use anyhow::{bail, Result};

use crate::domain::{Datetime, Prio, Tag, TodoItem, TodoItemCreate, TodoList};

pub struct AddOptions {
    pub task: Option<String>,
    pub prio: Option<Prio>,
    pub due: Option<Datetime>,
    pub tag: Option<Tag>,
}

impl TodoList {
    pub fn add_item(&self, repo: &impl TodoItemCreate, item: &TodoItem) -> Result<()> {
        if item.task.is_empty() {
            bail!("✘ Empty todo found")
        } else {
            repo.add(item)
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::cell::RefCell;

    use crate::domain::{Datetime, Prio, Status, Tag};

    struct FakeItemRepo {
        todos: RefCell<Vec<TodoItem>>,
    }

    impl FakeItemRepo {
        fn new() -> Self {
            Self {
                todos: RefCell::new(Vec::new()),
            }
        }

        fn len(&self) -> usize {
            self.todos.borrow().len()
        }
    }

    impl TodoItemCreate for FakeItemRepo {
        fn add(&self, item: &TodoItem) -> anyhow::Result<()> {
            self.todos.borrow_mut().push(item.clone());
            Ok(())
        }
    }

    #[test]
    fn should_err_on_an_empty_todo() -> Result<()> {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let item = TodoItem {
            id: "test-id".to_string(),
            task: String::new(),
            status: Status::Open,
            prio: Prio::Empty,
            tag: Tag::default(),
            due: Datetime::epoch(),
        };
        assert!(todo_list.add_item(&repo, &item).is_err());
        Ok(())
    }

    #[test]
    fn should_add_valid_todo() -> Result<()> {
        let repo = FakeItemRepo::new();
        assert_eq!(repo.len(), 0);
        let todo_list = TodoList::new();
        let item = TodoItem {
            id: "test-id".to_string(),
            task: "test-todo".to_string(),
            status: Status::Open,
            prio: Prio::Empty,
            tag: Tag::default(),
            due: Datetime::epoch(),
        };
        todo_list.add_item(&repo, &item)?;
        assert_eq!(repo.len(), 1);
        Ok(())
    }
}
