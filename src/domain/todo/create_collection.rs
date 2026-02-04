use anyhow::{Context, Result};

use crate::domain::{TodoList, TodoListSchema};

impl TodoList {
    pub fn create_collection(&self, repo: &impl TodoListSchema) -> Result<()> {
        repo.create_table()
            .context("✘ Couldn't create new collection")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    struct FakeListRepo {
        tables: RefCell<Vec<String>>,
    }

    impl FakeListRepo {
        fn new() -> Self {
            Self {
                tables: RefCell::new(Vec::new()),
            }
        }

        fn len(&self) -> usize {
            self.tables.borrow().len()
        }
    }

    impl TodoListSchema for FakeListRepo {
        fn create_table(&self) -> Result<()> {
            self.tables.borrow_mut().push("new-table".to_string());
            Ok(())
        }
    }

    struct FailingRepo;

    impl TodoListSchema for FailingRepo {
        fn create_table(&self) -> Result<()> {
            bail!("FakeError")
        }
    }

    #[test]
    fn should_create_a_new_table() -> Result<()> {
        let repo = FakeListRepo::new();
        let todo_list = TodoList::new();
        todo_list.create_collection(&repo)?;
        assert_eq!(repo.len(), 1);
        Ok(())
    }

    #[test]
    fn should_add_context_upon_failure() {
        let repo = FailingRepo;
        let todo_list = TodoList::new();
        let err = todo_list.create_collection(&repo).unwrap_err().to_string();
        assert!(err.contains("Couldn't create new collection"))
    }
}
