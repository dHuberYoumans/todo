use anyhow::{Context, Result};

use crate::domain::{Tag, TodoItemQueryColumns, TodoList};

impl TodoList {
    pub fn get_tags(&self, repo: &impl TodoItemQueryColumns) -> Result<Vec<Tag>> {
        repo.fetch_tags().context("✘ Couldn't fetch tags")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    struct FakeItemRepo {
        tags: RefCell<Vec<Tag>>,
    }

    struct FailingItemRepo;

    impl FakeItemRepo {
        fn new() -> Self {
            let tag_1 = Tag("tag-1".to_string());
            let tag_2 = Tag("tag-2".to_string());
            let tag_3 = Tag("tag-3".to_string());

            Self {
                tags: RefCell::new(vec![tag_1, tag_2, tag_3]),
            }
        }
    }

    impl TodoItemQueryColumns for FakeItemRepo {
        fn fetch_tags(&self) -> Result<Vec<Tag>> {
            Ok(self.tags.borrow().clone())
        }

        fn fetch_all_ids(&self) -> Result<Vec<String>> {
            unreachable!()
        }
    }

    impl TodoItemQueryColumns for FailingItemRepo {
        fn fetch_tags(&self) -> Result<Vec<Tag>> {
            bail!("Fake error while fetching tags")
        }

        fn fetch_all_ids(&self) -> Result<Vec<String>> {
            unreachable!()
        }
    }

    #[test]
    fn should_provide_context_upon_failure() {
        let repo = FailingItemRepo;
        let todo_list = TodoList::new();
        let err = todo_list.get_tags(&repo);
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't fetch tags"));
    }

    #[test]
    fn should_fetch_tags() {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let tags = todo_list.get_tags(&repo).unwrap();
        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0], Tag("tag-1".to_string()));
    }
}
