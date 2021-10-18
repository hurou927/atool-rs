mod archives;
mod helpers;
mod option;
mod ra_error;

use crate::{
    archives::archive::{archive_list, get_archive, LsParam},
    helpers::Helpers,
};
use archives::archive::{pack, PackParam};
use fs_extra::{
    dir::{move_dir, CopyOptions},
    move_items,
};
use glob::glob;
use option::{parse, SubCommand};
use std::{
    error::Error,
    path::{Path, PathBuf},
};
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
            println!("opt {:?}", input);
            let tmp_dir = tempfile::Builder::new().prefix("rapack-").tempdir()?;
            let tmp_dir_path = tmp_dir.path();
            let tmp_dir_str = tmp_dir_path.to_str().unwrap();
            let archive = get_archive(&input.source, &archives).await?;
            let src = PackParam {
                src_path: &input.source,
                dst_path: tmp_dir_str,
            };
            let output = archive.pack(&src).output().await?;
            let options = CopyOptions::new();

            let pattern = format!("{}/*", tmp_dir_str);

            let files: Vec<PathBuf> = glob(&pattern).unwrap().filter_map(Result::ok).collect();

            let can_copy = files.iter().all(|x| {
                let relative_path_from_tmp = x.strip_prefix(tmp_dir_path).unwrap();
                !input
                    .dest
                    .join(relative_path_from_tmp) //
                    .exists()
            });

            if can_copy {
                println!("Copy to {:?}", input.dest);
                move_items(&files, input.dest, &options)?;
            } else {
                println!(
                    "Files already exist. Copy from {:?} to {:?}",
                    tmp_dir_path, input.dest
                );
                move_dir(&tmp_dir, input.dest, &options)?;
            }

            println!("output: {}", tmp_dir_str);
            println!("{}", Helpers::to_string(&output.stdout)?);
        }
    };

    Ok(())
}
