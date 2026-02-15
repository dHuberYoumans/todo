use anyhow::{bail, Context, Result};

use crate::domain::{TodoList, TodoListCreate};

impl TodoList {
    pub fn add_list(&self, repo: &impl TodoListCreate, list: &str) -> Result<()> {
        if list.is_empty() {
            bail!("✘ Can't add a list without name")
        } else {
            repo.add(list).context("✘ Couldn't add new list")
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::Result;
    use std::cell::RefCell;

    use crate::domain::TodoListCreate;

    struct FakeListRepo {
        todo_lists: RefCell<Vec<String>>,
    }

    impl FakeListRepo {
        fn new() -> Self {
            Self {
                todo_lists: RefCell::new(Vec::new()),
            }
        }

        fn len(&self) -> usize {
            self.todo_lists.borrow().len()
        }
    }

    impl TodoListCreate for FakeListRepo {
        fn add(&self, list_name: &str) -> Result<()> {
            self.todo_lists.borrow_mut().push(list_name.to_string());
            Ok(())
        }
    }

    #[test]
    fn should_err_on_empty_list_name() {
        let repo = FakeListRepo::new();
        let todo_list = TodoList::new();
        assert!(todo_list.add_list(&repo, "").is_err());
    }

    #[test]
    fn should_add_new_list_for_non_empty_list_name() -> Result<()> {
        let repo = FakeListRepo::new();
        assert_eq!(repo.len(), 0);
        let todo_list = TodoList::new();
        todo_list.add_list(&repo, "test-new-list")?;
        assert_eq!(repo.len(), 1);
        Ok(())
    }
}
