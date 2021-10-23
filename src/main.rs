mod archives;
mod helpers;
mod option;
mod ra_error;

use crate::{
    archives::archive::{archive_list, get_archive, LsParam, UnPackParam},
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
    log::debug!("{:?}", opt);

    let archives = archive_list();

    match opt.subcmd {
        SubCommand::Ls(input) => {
            let a = get_archive(&input.source, &archives).await?;
            let src = LsParam {
                src_path: &input.source,
            };
            let output = a.ls(&src).output().await?;
            log::info!("{}", Helpers::to_string(&output.stdout)?);
        }

        SubCommand::Pack(input) => {
            let dest_path = Path::new(&input.dest);
            if dest_path.exists() {
                Err(RaError::DestinationAlreadyExists {
                    path: input.dest.clone(),
                })?
            }

            let mut srcs: Vec<&str> = Vec::new();
            let mut additional_srcs: Vec<&str> =
                input.additional_srcs.iter().map(|x| x.as_ref()).collect();
            srcs.push(&input.source);
            srcs.append(&mut additional_srcs);
            let param = PackParam {
                src_paths: srcs,
                dst_path: &input.dest,
            };
            let archive = get_archive(&input.dest, &archives).await?;
            let output = archive.pack(&param).output().await?;
            log::info!("{}", Helpers::to_string(&output.stdout)?);
        }
        SubCommand::Unpack(input) => {
            let tmp_dir = tempfile::Builder::new().prefix("rapack-").tempdir()?;
            let tmp_dir_path = tmp_dir.path();
            let tmp_dir_str = tmp_dir_path.to_str().ok_or(RaError::OtherPathError {
                reason: format!("temporary dir resolve error. {:?}", tmp_dir_path),
            })?;
            let archive = get_archive(&input.source, &archives).await?;
            let src = UnPackParam {
                src_path: &input.source,
                dst_path: tmp_dir_str,
            };
            let output = archive.unpack(&src).output().await?;
            log::info!("{}", Helpers::to_string(&output.stdout)?);

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
                log::info!("Extract to {:?}", input.dest);
                move_items(&files, input.dest, &options)?;
            } else {
                log::info!(
                    "Files already exist. Extract to {:?}",
                    input.dest.join(tmp_dir_path.file_name().unwrap())
                );
                move_dir(&tmp_dir, input.dest, &options)?;
            }
        }
    };

    Ok(())
}
