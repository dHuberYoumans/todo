use clap::ValueEnum;

#[derive(Debug, PartialEq, PartialOrd, ValueEnum, Clone, Copy)]
pub enum Status {
    Closed,
    Open,
}
