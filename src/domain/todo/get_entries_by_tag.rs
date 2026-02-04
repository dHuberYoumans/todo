use anyhow::{Context, Result};

use crate::domain::{ListFilter, Tag, TodoItem, TodoItemQuery, TodoList};

impl TodoList {
    pub fn get_entries_by_tag(
        &self,
        repo: &impl TodoItemQuery,
        tag: Tag,
        filter: Option<ListFilter>,
    ) -> Result<Vec<TodoItem>> {
        repo.fetch_by_tag(tag, filter)
            .context("✘ Couldn't fetch entries")
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::bail;
    use std::cell::RefCell;

    use crate::domain::{Datetime, Prio, Status, Tag};

    struct FakeItemRepo {
        todos: RefCell<Vec<TodoItem>>,
    }

    impl FakeItemRepo {
        fn new() -> Self {
            let todo_1 = TodoItem {
                id: "todo-1".to_string(),
                task: "task-1".to_string(),
                due: Datetime::epoch(),
                status: Status::Open,
                prio: Prio::Empty,
                tag: Tag("tag".to_string()),
            };
            let todo_2 = TodoItem {
                id: "todo-2".to_string(),
                task: "task-2".to_string(),
                due: Datetime::epoch(),
                status: Status::Closed,
                prio: Prio::Empty,
                tag: Tag("tag".to_string()),
            };
            let todo_3 = TodoItem {
                id: "todo-3".to_string(),
                task: "task-3".to_string(),
                due: Datetime::epoch(),
                status: Status::Open,
                prio: Prio::Empty,
                tag: Tag::empty(),
            };
            Self {
                todos: RefCell::new(vec![todo_1, todo_2, todo_3]),
            }
        }
    }

    struct FailingItemRepo;

    impl TodoItemQuery for FakeItemRepo {
        fn fetch_by_tag(
            &self,
            tag: crate::domain::Tag,
            filter: Option<ListFilter>,
        ) -> Result<Vec<TodoItem>> {
            let todos_by_tag: Vec<TodoItem> = self
                .todos
                .borrow()
                .iter()
                .filter(|todo| {
                    todo.tag == tag
                        && match filter {
                            Some(ListFilter::None) => true,
                            Some(ListFilter::Done) => todo.status == Status::Closed,
                            Some(ListFilter::Do) => todo.status == Status::Open,
                            None => true,
                        }
                })
                .cloned()
                .collect::<Vec<TodoItem>>();
            Ok(todos_by_tag)
        }

        fn fetch_by_prio(&self, _: crate::domain::Prio) -> Result<Vec<TodoItem>> {
            unreachable!()
        }

        fn fetch_task_by_id(&self, _: &str) -> Result<Option<String>> {
            unreachable!()
        }

        fn fetch_by_due_date(&self, _: i64, _: Option<ListFilter>) -> Result<Vec<TodoItem>> {
            unreachable!()
        }
    }

    impl TodoItemQuery for FailingItemRepo {
        fn fetch_by_tag(
            &self,
            _: crate::domain::Tag,
            _: Option<ListFilter>,
        ) -> Result<Vec<TodoItem>> {
            bail!("Fake error while fetching by tag")
        }

        fn fetch_by_prio(&self, _: crate::domain::Prio) -> Result<Vec<TodoItem>> {
            unreachable!()
        }

        fn fetch_task_by_id(&self, _: &str) -> Result<Option<String>> {
            unreachable!()
        }

        fn fetch_by_due_date(&self, _: i64, _: Option<ListFilter>) -> Result<Vec<TodoItem>> {
            unreachable!()
        }
    }

    #[test]
    fn should_provide_context_upon_failing() {
        let repo = FailingItemRepo;
        let todo_list = TodoList::new();
        let err = todo_list.get_entries_by_tag(&repo, Tag::empty(), None);
        assert!(err.is_err());
        let err_msg = err.unwrap_err().to_string();
        assert!(err_msg.contains("Couldn't fetch entries"));
    }

    #[test]
    fn should_fetch_all_todos_by_tag_for_no_filter() {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let todos_by_tag = todo_list
            .get_entries_by_tag(&repo, Tag("tag".to_string()), None)
            .unwrap();
        assert_eq!(todos_by_tag.len(), 2);
    }

    #[test]
    fn should_fetch_closed_todos_by_tag_for_filter_done() {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let todos_by_tag = todo_list
            .get_entries_by_tag(&repo, Tag("tag".to_string()), Some(ListFilter::Done))
            .unwrap();
        assert_eq!(todos_by_tag.len(), 1);
    }

    #[test]
    fn should_fetch_open_todos_by_tag_for_filter_do() {
        let repo = FakeItemRepo::new();
        let todo_list = TodoList::new();
        let todos_by_tag = todo_list
            .get_entries_by_tag(&repo, Tag("tag".to_string()), Some(ListFilter::Do))
            .unwrap();
        assert_eq!(todos_by_tag.len(), 1);
    }
}
