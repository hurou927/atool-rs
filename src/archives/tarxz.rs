use regex::Regex;
use tokio::process::Command;

use super::{
    archive::{Archive, ArchiveType, LsParam, PackParam, UnPackParam},
    tar_util::{self, Tar},
};

pub struct TarXz {
    filename_regex: Regex,
    file_cmd_regex: Regex,
    archive_type: ArchiveType,
}

impl TarXz {
    pub fn new() -> Self {
        let r_filename = r"^.*\.tar\.xz$";

        let r_file_cmd = r#"^.*tar archive.*XZ compressed data.*"#;

        TarXz {
            filename_regex: Regex::new(r_filename).unwrap(),
            file_cmd_regex: Regex::new(r_file_cmd).unwrap(),
            archive_type: ArchiveType::TarXz,
        }
    }
}

impl Archive for TarXz {
    fn get_type(&self) -> super::archive::ArchiveType {
        self.archive_type
    }
    fn from_filename(&self, filename: &str) -> bool {
        self.filename_regex.is_match(filename)
    }

    fn from_file_cmd(&self, stdout: &str) -> bool {
        self.file_cmd_regex.is_match(stdout)
    }

    fn ls(&self, param: &LsParam) -> Command {
        tar_util::list(&param.src_path)
    }
    fn pack(&self, param: &PackParam) -> Command {
        tar_util::compress(&param.src_paths, param.dst_path, Tar::Xz)
    }
    fn unpack(&self, param: &UnPackParam) -> Command {
        tar_util::uncompress(param.src_path, param.dst_path)
    }
}

#[cfg(test)]
mod tests {

    use crate::archives::{archive::Archive, tarxz::TarXz, test_util::TestUtil};

    #[test]
    fn test_from_filename() {
        let archive = TarXz::new();
        assert_eq!(archive.from_filename("/tmp/hoge.tar.xz"), true);
        assert_eq!(archive.from_filename("/tmp/hoge.tar.bz2"), false);
        assert_eq!(archive.from_filename("/tmp/hoge.xz"), false);
        assert_eq!(archive.from_filename("/tmp/hogetar..xz"), false);
    }

    fn from_file_cmd(path: &str) -> bool {
        let archive = TarXz::new();
        let file_path = TestUtil::resource_path(path);
        let result = TestUtil::file_cmd_result(&file_path);
        archive.from_file_cmd(&result)
    }

    #[test]
    fn test_from_file_cmd() {
        assert_eq!(from_file_cmd("test_dir.tar.xz"), true);
        assert_eq!(from_file_cmd("test_dir.tar.bz2"), false);
        // assert_eq!(from_file_cmd("test_dir.xz"), false);
    }
}
