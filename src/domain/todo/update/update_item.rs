use anyhow::{Context, Result};

use crate::domain::{Datetime, Prio, Status, Tag, TodoItemUpdate, TodoList};

impl TodoList {
    pub fn update_item(
        &self,
        repo: &impl TodoItemUpdate,
        due: Option<Datetime>,
        prio: Option<Prio>,
        status: Option<Status>,
        tag: Option<Tag>,
        ids: Vec<String>,
    ) -> Result<()> {
        repo.update(due, prio, status, tag, ids)
            .context("✘ Couldn't update items")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    use crate::domain::TodoItem;

    struct FakeItemRepo {
        todos: RefCell<Vec<TodoItem>>,
    }

    struct FailingRepo;

    impl FakeItemRepo {
        fn new() -> Self {
            let todo_one = TodoItem {
                id: "test-id-1".to_string(),
                task: "some-task-1".to_string(),
                status: Status::Open,
                due: Datetime::epoch(),
                tag: Tag("some-tag-1".to_string()),
                prio: Prio::P3,
            };
            let todo_two = TodoItem {
                id: "test-id-2".to_string(),
                task: "some-task-2".to_string(),
                status: Status::Open,
                due: Datetime::epoch(),
                tag: Tag("some-tag-2".to_string()),
                prio: Prio::P3,
            };

            Self {
                todos: RefCell::new(vec![todo_one, todo_two]),
            }
        }

        fn get_todo(&self) -> Vec<TodoItem> {
            self.todos.borrow().clone()
        }
    }

    impl TodoItemUpdate for FakeItemRepo {
        fn update(
            &self,
            due: Option<Datetime>,
            prio: Option<Prio>,
            status: Option<Status>,
            tag: Option<Tag>,
            ids: Vec<String>,
        ) -> Result<()> {
            let mut todos = self.todos.borrow_mut();
            for todo in todos.iter_mut() {
                if ids.contains(&todo.id) {
                    if let Some(due) = due {
                        todo.due = due;
                    };
                    if let Some(prio) = prio {
                        todo.prio = prio;
                    };
                    if let Some(status) = status {
                        todo.status = status;
                    };
                    if let Some(tag) = tag.clone() {
                        todo.tag = tag;
                    };
                }
            }
            Ok(())
        }

        fn close_all(&self, _: Option<Prio>) -> Result<()> {
            unreachable!()
        }

        fn update_task(&self, _: &str, _: &str) -> Result<()> {
            unreachable!()
        }
    }

    impl TodoItemUpdate for FailingRepo {
        fn update(
            &self,
            _: Option<Datetime>,
            _: Option<Prio>,
            _: Option<Status>,
            _: Option<Tag>,
            _: Vec<String>,
        ) -> Result<()> {
            bail!("Fake error while updating item")
        }

        fn close_all(&self, _: Option<Prio>) -> Result<()> {
            unreachable!()
        }

        fn update_task(&self, _: &str, _: &str) -> Result<()> {
            unreachable!()
        }
    }

    #[test]
    fn should_provide_context_upon_failure() {
        let repo = FailingRepo;
        let todo_list = TodoList::new();
        let err = todo_list.update_item(&repo, None, None, None, None, vec!["test-id".to_string()]);
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't update items"))
    }

    #[test]
    fn should_update_a_single_item() -> Result<()> {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let ids = vec!["test-id-1".to_string()];
        let new_tag = Tag("changed_tag".to_string());
        let old_tag = Tag("some-tag-2".to_string());
        todo_list.update_item(&repo, None, None, None, Some(new_tag.clone()), ids)?;
        assert_eq!(repo.get_todo()[0].tag, new_tag);
        assert_eq!(repo.get_todo()[1].tag, old_tag);
        Ok(())
    }

    #[test]
    fn should_update_multiple_items() -> Result<()> {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let ids = vec!["test-id-1".to_string(), "test-id-2".to_string()];
        todo_list.update_item(&repo, None, Some(Prio::P1), None, None, ids)?;
        assert_eq!(repo.get_todo()[0].prio, Prio::P1);
        assert_eq!(repo.get_todo()[1].prio, Prio::P1);
        Ok(())
    }
}
