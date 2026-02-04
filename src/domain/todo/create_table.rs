use anyhow::{Context, Result};

use crate::domain::{TodoItemSchema, TodoList};

impl TodoList {
    pub fn create_table(&self, repo: &impl TodoItemSchema) -> Result<()> {
        repo.create_table().context("✘ Couldn't create new table")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    struct FakeItemRepo {
        tables: RefCell<Vec<String>>,
    }

    impl FakeItemRepo {
        fn new() -> Self {
            Self {
                tables: RefCell::new(Vec::new()),
            }
        }

        fn len(&self) -> usize {
            self.tables.borrow_mut().len()
        }
    }

    impl TodoItemSchema for FakeItemRepo {
        fn create_table(&self) -> Result<()> {
            self.tables.borrow_mut().push("new-todo-list".to_string());
            Ok(())
        }
    }

    struct FailingRepo;

    impl TodoItemSchema for FailingRepo {
        fn create_table(&self) -> Result<()> {
            bail!("Fake error")
        }
    }

    #[test]
    fn should_add_new_todo_list() -> Result<()> {
        let repo = FakeItemRepo::new();
        assert_eq!(repo.len(), 0);
        let todo_list = TodoList::new();
        todo_list.create_table(&repo)?;
        assert_eq!(repo.len(), 1);
        Ok(())
    }

    #[test]
    fn should_provide_context_upon_failure() {
        let repo = FailingRepo;
        let todo_list = TodoList::new();
        let err = todo_list.create_table(&repo).unwrap_err().to_string();
        assert!(err.contains("Couldn't create new table"));
    }
}
