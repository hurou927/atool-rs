mod archives;
mod helpers;
mod ra_error;

use std::error::Error;

use crate::{
    archives::archive::{archive_list, get_archive, Src},
    helpers::Helpers,
};
use clap::Clap;

/// A basic example
#[derive(Clap, Debug, Clone)]
#[clap(
    name = "ratool",
    version = "1.0",
    author = "hurou927 <god.be.with.ye.fs@gmail.com>"
)]
struct Opt {
    #[clap(subcommand)]
    subcmd: SubCommand,
}
#[derive(Clap, Debug, Clone)]
enum SubCommand {
    Ls(Ls),
}

/// A subcommand for controlling testing
#[derive(Clap, Debug, Clone)]
struct Ls {
    /// Print debug info
    #[clap(short)]
    debug: bool,

    source: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    let archives = archive_list();

    match opt.subcmd {
        SubCommand::Ls(input) => {
            let a = get_archive(&input.source, &archives).await?;
            let src = Src {
                src_path: &input.source,
            };
            let output = a.ls(&src).output().await?;
            println!("{}", Helpers::to_string(&output.stdout)?);
        }
    };

    Ok(())
}
