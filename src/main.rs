mod archives;
mod helpers;
mod option;
mod ra_error;

use crate::{
    archives::archive::{archive_list, get_archive, LsParam},
    helpers::Helpers,
};
use option::{parse, SubCommand};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = parse();

    let archives = archive_list();

    match opt.subcmd {
        SubCommand::Ls(input) => {
            let a = get_archive(&input.source, &archives).await?;
            let src = LsParam {
                src_path: &input.source,
            };
            let output = a.ls(&src).output().await?;
            println!("{}", Helpers::to_string(&output.stdout)?);
        }
    };

    Ok(())
}
