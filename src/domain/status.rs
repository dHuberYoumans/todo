use clap::ValueEnum;

#[derive(Hash, Debug, PartialEq, PartialOrd, ValueEnum, Clone, Copy)]
pub enum Status {
    Closed,
    Open,
}
