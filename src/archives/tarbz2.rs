use regex::Regex;
use tokio::process::Command;

use super::archive::{Archive, Src};

pub struct TarBz2<'a> {
    filename_suffix: &'a str,
    file_cmd_result: Regex,
}

impl<'a> TarBz2<'a> {
    pub fn new() -> Self {
        TarBz2 {
            filename_suffix: "hoge",
            file_cmd_result: Regex::new("^hoge$").unwrap(),
        }
    }
}

impl<'a> Archive for TarBz2<'a> {
    fn from_filename(&self, filename: &str) -> bool {
        filename.ends_with(self.filename_suffix)
    }

    fn from_file_cmd(&self, stdout: &str) -> bool {
        self.file_cmd_result.is_match(stdout)
    }

    fn ls(&self, src: Src) -> Command {
        todo!()
    }
    fn pack(&self, src: Src) -> Command {
        todo!()
    }
}
