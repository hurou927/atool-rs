mod archives;
mod helpers;
mod option;
mod ra_error;

use crate::{
    archives::archive::{archive_list, get_archive, LsParam},
    helpers::Helpers,
    ra_error::RaError,
};
use archives::archive::PackParam;
use fs_extra::{
    dir::{move_dir, CopyOptions},
    move_items,
};
use glob::glob;
use option::{parse, SubCommand};
use std::{
    error::Error,
    path::{Path, PathBuf, StripPrefixError},
};

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
            log::debug!("{}", Helpers::to_string(&output.stdout)?);
        }

        SubCommand::Pack(_input) => {
            todo!()
        }
        SubCommand::Unpack(input) => {
            log::debug!("opt {:?}", input);
            let tmp_dir = tempfile::Builder::new().prefix("rapack-").tempdir()?;
            let tmp_dir_path = tmp_dir.path();
            let tmp_dir_str = tmp_dir_path.to_str().ok_or(RaError::OtherPathError {
                reason: format!("temporary dir resolve error. {:?}", tmp_dir_path),
            })?;
            let archive = get_archive(&input.source, &archives).await?;
            let src = PackParam {
                src_path: &input.source,
                dst_path: tmp_dir_str,
            };
            let output = archive.pack(&src).output().await?;

            let glob_pattern = format!("{}/*", tmp_dir_str);
            let files: Vec<PathBuf> = glob(&glob_pattern)?.filter_map(Result::ok).collect();

            let relative_paths_from_tmp = files
                .iter()
                .map(|x| x.strip_prefix(tmp_dir_path))
                .collect::<Result<Vec<&Path>, StripPrefixError>>()?;

            let can_copy = relative_paths_from_tmp
                .iter()
                .all(|x| !input.dest.join(x).exists());

            let options = CopyOptions::new();
            if can_copy {
                log::debug!("Copy to {:?}", input.dest);
                move_items(&files, input.dest, &options)?;
            } else {
                log::debug!(
                    "Files already exist. Copy from {:?} to {:?}",
                    tmp_dir_path,
                    input.dest
                );
                move_dir(&tmp_dir, input.dest, &options)?;
            }

            log::info!("output: {}", tmp_dir_str);
            log::info!("{}", Helpers::to_string(&output.stdout)?);
        }
    };

    Ok(())
}
