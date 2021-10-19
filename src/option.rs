use std::path::PathBuf;

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
    #[clap(short = 'l', long = "loglevel", default_value = "info")]
    pub log_level: String,
}
#[derive(Clap, Debug, Clone)]
pub enum SubCommand {
    Ls(Ls),
    Pack(Pack),
    Unpack(Unpack),
}

#[derive(Clap, Debug, Clone)]
pub struct Ls {
    /// Print debug info
    #[clap(short)]
    pub debug: bool,

    pub source: String,
}

#[derive(Clap, Debug, Clone)]
pub struct Pack {
    pub source: String,
}

#[derive(Clap, Debug, Clone)]
pub struct Unpack {
    pub source: String,
    #[clap(
        short = 'X',
        long = "extract",
        parse(from_os_str),
        default_value = "./"
    )]
    pub dest: PathBuf,
}

pub fn parse() -> Opt {
    let opt = Opt::parse();
    match opt.log_level.as_ref() {
        "trace" => log::set_max_level(log::LevelFilter::Trace),
        "debug" => log::set_max_level(log::LevelFilter::Debug),
        _ => log::set_max_level(log::LevelFilter::Warn),
    };
    opt
}
