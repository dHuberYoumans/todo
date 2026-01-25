use anyhow::Result;

pub trait Editor {
    fn edit(&self, old_text: Option<String>) -> Result<String>;
}
