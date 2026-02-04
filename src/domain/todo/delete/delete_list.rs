use anyhow::{Context, Result};

use crate::domain::{TodoList, TodoListDelete};

impl TodoList {
    pub fn delete_list(&self, repo: &impl TodoListDelete, list: &str) -> Result<()> {
        repo.delete(list)
            .context(format!("✘ Couldn't delete list '{list}'"))
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
            let list_1 = "todo-list-one".to_string();
            let list_2 = "todo-list-two".to_string();
            Self {
                collection: RefCell::new(vec![list_1, list_2]),
            }
        }

        fn len(&self) -> usize {
            self.collection.borrow().len()
        }
    }

    struct FailingListRepo;

    impl TodoListDelete for FakeListRepo {
        fn delete(&self, list_name: &str) -> Result<()> {
            let mut collection = self.collection.borrow_mut();
            collection.retain(|list| list == list_name);
            Ok(())
        }
    }

    impl TodoListDelete for FailingListRepo {
        fn delete(&self, _: &str) -> Result<()> {
            bail!("Fake error while deleting a list from collection")
        }
    }

    #[test]
    fn should_provide_context_upon_failure() {
        let repo = FailingListRepo;
        let todo_list = TodoList::new();
        let err = todo_list.delete_list(&repo, "todo-list-one");
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't delete list"));
    }

    #[test]
    fn should_delete_list_from_collection_by_name() -> Result<()> {
        let repo = FakeListRepo::new();
        let todo_list = TodoList::new();
        assert_eq!(repo.len(), 2);
        todo_list.delete_list(&repo, "todo-list-one")?;
        assert_eq!(repo.len(), 1);
        Ok(())
    }
}
