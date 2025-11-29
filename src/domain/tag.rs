use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Tag(pub String);

#[derive(Error, Debug)]
pub enum TagParseError {
    #[error("Invalid tag. Expected string without spaces")]
    InvalidFormat,
}

impl FromStr for Tag {
    type Err = TagParseError;

    fn from_str(tag: &str) -> Result<Self, Self::Err> {
        if tag.contains(' ') {
            return Err(TagParseError::InvalidFormat);
        }
        Ok(Tag(tag.to_string()))
    }
}

impl Tag {
    pub fn empty() -> Tag {
        Tag(String::new())
    }
}
