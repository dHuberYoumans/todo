use anyhow::{Context, Result};

use crate::domain::{TodoItemSchema, TodoList};

impl TodoList {
    pub fn create_table(&self, repo: &impl TodoItemSchema, name: Option<&str>) -> Result<()> {
        repo.create_table(name)
            .context("✘ Couldn't create new table")
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

        fn get_tables(&self) -> Vec<String> {
            self.tables.borrow().clone()
        }
    }

    impl TodoItemSchema for FakeItemRepo {
        fn create_table(&self, name: Option<&str>) -> Result<()> {
            let name = name.unwrap_or("default");
            self.tables.borrow_mut().push(name.to_string());
            Ok(())
        }
    }

    struct FailingRepo;

    impl TodoItemSchema for FailingRepo {
        fn create_table(&self, _: Option<&str>) -> Result<()> {
            bail!("Fake error while creating table")
        }
    }

    #[test]
    fn should_add_new_named_todo_list() -> Result<()> {
        let repo = FakeItemRepo::new();
        assert_eq!(repo.len(), 0);
        let todo_list = TodoList::new();
        let name = "new-list";
        todo_list.create_table(&repo, Some(name))?;
        assert_eq!(repo.len(), 1);
        let collection = repo.get_tables();
        assert!(collection.contains(&name.to_string()));
        Ok(())
    }

    #[test]
    fn should_add_new_todo_list_with_default_name() -> Result<()> {
        let repo = FakeItemRepo::new();
        assert_eq!(repo.len(), 0);
        let todo_list = TodoList::new();
        todo_list.create_table(&repo, None)?;
        let default = "default".to_string();
        assert_eq!(repo.len(), 1);
        let collection = repo.get_tables();
        assert!(collection.contains(&default));
        Ok(())
    }

    #[test]
    fn should_provide_context_upon_failure() {
        let repo = FailingRepo;
        let todo_list = TodoList::new();
        let err = todo_list.create_table(&repo, None).unwrap_err().to_string();
        assert!(err.contains("Couldn't create new table"));
    }
}
