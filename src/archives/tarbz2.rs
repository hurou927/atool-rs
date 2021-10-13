use regex::Regex;
use tokio::process::Command;

use super::archive::{Archive, Src};

pub struct TarBz2 {
    filename_regex: Regex,
    file_cmd_regex: Regex,
}

impl TarBz2 {
    pub fn new() -> Self {
        let r_filename = r".*\.tar\.bz2$";

        let r_file_cmd = r#"^(GNU|POSIX) tar archive \(bzip2 compressed data(\W|$)/"#;

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
    use crate::archives::{archive::Archive, tarbz2::TarBz2};

    #[test]
    fn from_filename() {
        let archive = TarBz2::new();
        assert_eq!(archive.from_filename("/tmp/hoge.tar.bz2"), true);
        assert_eq!(archive.from_filename("/tmp/hoge.tar.gz"), false);
        assert_eq!(archive.from_filename("/tmp/hoge.bz2"), false);
        assert_eq!(archive.from_filename("/tmp/hogetar..bz2"), false);
    }
}
