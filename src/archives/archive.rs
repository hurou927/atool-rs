use tokio::process::Command;

use crate::archives::tarbz2::TarBz2;

pub struct Src<'a> {
    pub src_path: &'a str,
}

pub trait Archive {
    fn from_filename(&self, filename: &str) -> bool;
    fn from_file_cmd(&self, stdout: &str) -> bool;
    fn ls(&self, src: &Src) -> Command;
    fn pack(&self, src: Src) -> Command;
}

pub fn archive_list() -> Vec<Box<dyn Archive>> {
    vec![Box::new(TarBz2::new())]
}
