use clap::ValueEnum;
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Default, ValueEnum)]
pub enum Prio {
    #[value(aliases = ["P1","p1","1","high"])]
    P1,
    #[value(aliases = ["P2","p2","2","mid"])]
    P2,
    #[value(aliases = ["P3","p3","3","low"])]
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
