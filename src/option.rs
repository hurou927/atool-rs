use clap::Clap;

/// A basic example
#[derive(Clap, Debug, Clone)]
#[clap(
    name = "ratool",
    version = "1.0",
    author = "hurou927 <god.be.with.ye.fs@gmail.com>"
)]
pub struct Opt {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}
#[derive(Clap, Debug, Clone)]
pub enum SubCommand {
    Ls(Ls),
}

/// A subcommand for controlling testing
#[derive(Clap, Debug, Clone)]
pub struct Ls {
    /// Print debug info
    #[clap(short)]
    pub debug: bool,

    pub source: String,
}

pub fn parse() -> Opt {
    Opt::parse()
}
