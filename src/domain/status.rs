use clap::ValueEnum;

#[derive(Debug, PartialEq, PartialOrd, ValueEnum, Clone)]
pub enum Status {
    Closed,
    Open,
}
