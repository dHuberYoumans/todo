use clap::ValueEnum;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Default, ValueEnum)]
pub enum Prio {
    P1,
    P2,
    P3,
    #[default]
    Empty,
}
