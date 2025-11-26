use clap::ValueEnum;
use std::fmt;

#[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Default, ValueEnum)]
pub enum Prio {
    #[value(alias = "P1")]
    P1,
    #[value(alias = "P2")]
    P2,
    #[value(alias = "P3")]
    P3,
    #[default]
    Empty,
}

impl fmt::Display for Prio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prio::P1 => write!(f, "P1"),
            Prio::P2 => write!(f, "P2"),
            Prio::P3 => write!(f, "P3"),
            Prio::Empty => write!(f, ""),
        }
    }
}
