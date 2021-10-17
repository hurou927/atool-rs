mod archives;
mod helpers;
mod ra_error;

use std::error::Error;

use archives::archive::Archive;
use ra_error::RaError;
use tokio::process::Command;

use crate::{
    archives::archive::{archive_list, Src},
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

async fn from_file_cmd<'a>(
    file_path: &str,
    archives: &'a Vec<Box<dyn Archive>>,
) -> Option<&'a Box<dyn Archive>> {
    let output = Command::new("file")
        .arg("-z")
        .arg("-L")
        .arg("-b")
        .arg(file_path)
        .output()
        .await
        .unwrap();
    let result = std::str::from_utf8(&output.stdout).unwrap();
    archives.iter().find(|x| x.from_file_cmd(result))
}

async fn get_archive<'a>(
    file_path: &'a str,
    archives: &'a Vec<Box<dyn Archive>>,
) -> Result<&'a Box<dyn Archive>, RaError> {
    match archives.iter().find(|x| x.from_filename(file_path)) {
        Some(x) => Ok(x),
        None => from_file_cmd(file_path, archives)
            .await
            .ok_or(RaError::UnSupportedFormat {
                path: file_path.to_string(),
            }),
    }
}

fn to_string(bin: &Vec<u8>) -> Result<String, Box<dyn Error>> {
    let a = std::str::from_utf8(bin)?;
    Ok(a.to_string())
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
