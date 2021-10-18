mod archives;
mod helpers;
mod option;
mod ra_error;

use crate::{
    archives::archive::{archive_list, get_archive, LsParam},
    helpers::Helpers,
};
use archives::archive::{pack, PackParam};
use option::{parse, SubCommand};
use std::error::Error;
use tempfile::tempdir;

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

        SubCommand::Pack(input) => {
            todo!()
        }
        SubCommand::Unpack(input) => {
            let tmp_dir = tempfile::Builder::new().prefix("rapack-").tempdir()?;
            let tmp_dir_str = tmp_dir.path().to_str().unwrap();
            let a = get_archive(&input.source, &archives).await?;
            let src = PackParam {
                src_path: &input.source,
                dst_path: tmp_dir_str,
            };
            let output = a.pack(&src).output().await?;

            println!("output: {}", tmp_dir_str);
            println!("{}", Helpers::to_string(&output.stdout)?);
        }
    };

    Ok(())
}
