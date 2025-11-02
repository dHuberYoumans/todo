use anyhow::Result;

use crate::domain::{TodoItemRepository, TodoList, TodoListRepository};

impl TodoList {
    pub fn new_list(
        &mut self,
        todo_list_repo: &impl TodoListRepository,
        todo_item_repo: &impl TodoItemRepository,
        list: String,
        checkout: bool,
    ) -> Result<()> {
        println!("⧖ Creating new_list..");
        todo_list_repo.add(&list)?;
        todo_item_repo.create_table()?;
        println!("✔ Created new list '{list}'");
        if checkout {
            log::info!("checking out list '{list}'");
            self.load(todo_list_repo, list.clone())?;
            println!("✔ Now using '{list}'");
        };
        Ok(())
    }
}
