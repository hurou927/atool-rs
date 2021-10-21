use clap::Clap;
use env_logger::Builder;
use env_logger::Env;
use std::io::Write;
use std::path::PathBuf;

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
    #[clap(short = 'l', long = "loglevel", default_value = "warn")]
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
    pub dest: String,
    pub source: String,
    pub additional_srcs: Vec<String>,
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
pub fn init_logger() {
    let env = Env::default()
        .filter("RATOOL_LOG")
        .write_style("RATOOL_LOG");
    let mut builder = Builder::from_env(env);

    builder
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        // .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        // .filter(None, LevelFilter::Info)
        .init();
}

pub fn parse() -> Opt {
    let opt = Opt::parse();
    init_logger();
    // log::trace!("some trace log");
    // log::debug!("some debug log");
    // log::info!("some information log");
    // log::warn!("some warning log");
    // log::error!("some error log");
    opt
}
