use regex::Regex;
use tokio::process::Command;

use super::archive::{Archive, Src};

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

    fn ls(&self, src: Src) -> Command {
        todo!()
    }
    fn pack(&self, src: Src) -> Command {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::archives::{
        archive::{self, Archive},
        tarbz2::TarBz2,
        test_util::TestUtil,
    };

    #[test]
    fn from_filename() {
        let archive = TarBz2::new();
        assert_eq!(archive.from_filename("/tmp/hoge.tar.bz2"), true);
        assert_eq!(archive.from_filename("/tmp/hoge.tar.gz"), false);
        assert_eq!(archive.from_filename("/tmp/hoge.bz2"), false);
        assert_eq!(archive.from_filename("/tmp/hogetar..bz2"), false);
    }

    #[test]
    fn from_file_cmd() {
        let archive = TarBz2::new();
        let file_path = TestUtil::resource_path("test_dir.tar.bz2");
        let result = TestUtil::file_cmd_result(&file_path);
        println!("result?: {}", result);
        assert_eq!(archive.from_file_cmd(&result), true);
    }
}
