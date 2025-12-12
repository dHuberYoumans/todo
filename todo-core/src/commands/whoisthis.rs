use anyhow::Result;

use crate::domain::TodoList;

impl TodoList {
    pub fn whoisthis(&self) -> Result<()> {
        let current = std::env::var("CURRENT")?;
        if current.is_empty() {
            eprintln!("✘ Currently, no list is active");
        } else {
            println!("This is {current}. Ready for duty!");
        }
        Ok(())
    }
}
