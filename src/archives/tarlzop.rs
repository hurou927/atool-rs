use regex::Regex;
use tokio::process::Command;

use super::{
    archive::{Archive, ArchiveType, LsParam, PackParam, UnPackParam},
    tar_util::{self, Tar},
};

pub struct TarLzop {
    filename_regex: Regex,
    file_cmd_regex: Regex,
    archive_type: ArchiveType,
}

impl TarLzop {
    pub fn new() -> Self {
        let r_filename = r"^.*\.tar\.lzo$";

        let r_file_cmd = r#"lzop compressed data.*"#;

        TarLzop {
            filename_regex: Regex::new(r_filename).unwrap(),
            file_cmd_regex: Regex::new(r_file_cmd).unwrap(),
            archive_type: ArchiveType::TarLzop,
        }
    }
}

impl Archive for TarLzop {
    fn get_type(&self) -> super::archive::ArchiveType {
        self.archive_type
    }
    fn from_filename(&self, filename: &str) -> bool {
        self.filename_regex.is_match(filename)
    }

    // always false
    // $ file tests/resource/test_dir.tar.lzo
    // tests/resource/test_dir.tar.lzo: lzop compressed data - version 1.030, LZO1X-1, os: Unix
    fn from_file_cmd(&self, stdout: &str) -> bool {
        // self.file_cmd_regex.is_match(stdout)
        false
    }

    fn ls(&self, param: &LsParam) -> Command {
        tar_util::list(&param.src_path)
    }
    fn pack(&self, param: &PackParam) -> Command {
        tar_util::compress(&param.src_paths, param.dst_path, Tar::Gz)
    }
    fn unpack(&self, param: &UnPackParam) -> Command {
        tar_util::uncompress(param.src_path, param.dst_path)
    }
}

#[cfg(test)]
mod tests {

    use crate::archives::{archive::Archive, tarlzop::TarLzop, test_util::TestUtil};

    #[test]
    fn test_from_filename() {
        let archive = TarLzop::new();
        assert_eq!(archive.from_filename("/tmp/hoge.tar.lzo"), true);
        assert_eq!(archive.from_filename("/tmp/hoge.tar.lzop"), false);
        assert_eq!(archive.from_filename("/tmp/hoge.tar.bz2"), false);
        assert_eq!(archive.from_filename("/tmp/hoge.lzo"), false);
        assert_eq!(archive.from_filename("/tmp/hogetar..lzo"), false);
    }

    fn from_file_cmd(path: &str) -> bool {
        let archive = TarLzop::new();
        let file_path = TestUtil::resource_path(path);
        let result = TestUtil::file_cmd_result(&file_path);
        archive.from_file_cmd(&result)
    }

    #[test]
    fn test_from_file_cmd() {
        assert_eq!(from_file_cmd("test_dir.tar.lzo"), false); // always false
        assert_eq!(from_file_cmd("test_dir.tar.bz2"), false);
        // assert_eq!(from_file_cmd("test_dir.lzop"), false);
    }
}
