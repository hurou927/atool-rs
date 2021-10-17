use regex::Regex;
use tokio::process::Command;

use super::archive::{Archive, LsParam, PackParam, UnPackParam};

pub struct TarBz2 {
    filename_regex: Regex,
    file_cmd_regex: Regex,
}

impl TarBz2 {
    pub fn new() -> Self {
        let r_filename = r"^.*\.tar\.bz2$";

        let r_file_cmd = r#"^.*tar archive.*bzip2 compressed data.*"#;

        TarBz2 {
            filename_regex: Regex::new(r_filename).unwrap(),
            file_cmd_regex: Regex::new(r_file_cmd).unwrap(),
        }
    }
}

impl Archive for TarBz2 {
    fn from_filename(&self, filename: &str) -> bool {
        self.filename_regex.is_match(filename)
    }

    fn from_file_cmd(&self, stdout: &str) -> bool {
        self.file_cmd_regex.is_match(stdout)
    }

    fn ls(&self, param: &LsParam) -> Command {
        let mut cmd = Command::new("tar");
        cmd.arg("-tvf").arg(param.src_path);
        cmd
    }
    fn pack(&self, param: &PackParam) -> Command {
        todo!()
    }
    fn unpack(&self, param: &UnPackParam) -> Command {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::archives::{archive::Archive, tarbz2::TarBz2, test_util::TestUtil};

    #[test]
    fn test_from_filename() {
        let archive = TarBz2::new();
        assert_eq!(archive.from_filename("/tmp/hoge.tar.bz2"), true);
        assert_eq!(archive.from_filename("/tmp/hoge.tar.gz"), false);
        assert_eq!(archive.from_filename("/tmp/hoge.bz2"), false);
        assert_eq!(archive.from_filename("/tmp/hogetar..bz2"), false);
    }

    fn from_file_cmd(path: &str) -> bool {
        let archive = TarBz2::new();
        let file_path = TestUtil::resource_path(path);
        let result = TestUtil::file_cmd_result(&file_path);
        archive.from_file_cmd(&result)
    }

    #[test]
    fn test_from_file_cmd() {
        assert_eq!(from_file_cmd("test_dir.tar.bz2"), true);
        assert_eq!(from_file_cmd("test_dir.tar.gz"), false);
        assert_eq!(from_file_cmd("test_dir.bz2"), false);
    }
}
