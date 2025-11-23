use clap::ValueEnum;

#[derive(Hash, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Default, ValueEnum)]
pub enum Prio {
    P1,
    P2,
    P3,
    #[default]
    Empty,
}
