use anyhow::{Context, Result};

use crate::domain::{TodoList, TodoListRead};

impl TodoList {
    pub fn get_collection(&self, repo: &impl TodoListRead) -> Result<Vec<String>> {
        repo.fetch_all().context("✘ Couldn't fetch collection")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    struct FakeListRepo {
        collection: RefCell<Vec<String>>,
    }

    impl FakeListRepo {
        fn new() -> Self {
            Self {
                collection: RefCell::new(vec![
                    "todo-list-1".to_string(),
                    "todo-list-2".to_string(),
                ]),
            }
        }
    }

    impl TodoListRead for FakeListRepo {
        fn fetch_all(&self) -> Result<Vec<String>> {
            Ok(self.collection.borrow().clone())
        }

        fn fetch_id(&self, _: &str) -> Result<i64> {
            unreachable!()
        }
    }

    struct FailingListRepo;

    impl TodoListRead for FailingListRepo {
        fn fetch_all(&self) -> Result<Vec<String>> {
            bail!("Fake error when fetching collection")
        }

        fn fetch_id(&self, _: &str) -> Result<i64> {
            unreachable!()
        }
    }

    #[test]
    fn should_fetch_all_todo_lists_in_collection() -> Result<()> {
        let repo = FakeListRepo::new();
        let todo_list = TodoList::new();
        let collection = todo_list.get_collection(&repo)?;
        assert_eq!(collection.len(), 2);
        Ok(())
    }

    #[test]
    fn should_provide_context_upon_failure() {
        let repo = FailingListRepo;
        let todo_list = TodoList::new();
        let collection = todo_list.get_collection(&repo);
        assert!(collection.is_err());
        let err = collection.unwrap_err().to_string();
        assert!(err.contains("Couldn't fetch collection"));
    }
}
