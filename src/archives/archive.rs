use tokio::process::Command;

use crate::{archives::tarbz2::TarBz2, ra_error::RaError};

pub struct LsParam<'a> {
    pub src_path: &'a str,
}

pub struct PackParam<'a> {
    pub src_path: &'a str,
    pub dst_path: Option<&'a str>,
}
pub struct UnPackParam<'a> {
    pub src_path: &'a str,
    pub dst_path: Option<&'a str>,
}

pub trait Archive {
    fn from_filename(&self, filename: &str) -> bool;
    fn from_file_cmd(&self, stdout: &str) -> bool;
    fn ls(&self, src: &LsParam) -> Command;
    fn pack(&self, src: &PackParam) -> Command;
    fn unpack(&self, src: &UnPackParam) -> Command;
}

pub fn archive_list() -> Vec<Box<dyn Archive>> {
    vec![Box::new(TarBz2::new())]
}

async fn detect_by_file_cmd<'a>(
    file_path: &str,
    archives: &'a Vec<Box<dyn Archive>>,
) -> Option<&'a Box<dyn Archive>> {
    let output = Command::new("file")
        .arg("-z")
        .arg("-L")
        .arg("-b")
        .arg(file_path)
        .output()
        .await
        .unwrap();
    let result = std::str::from_utf8(&output.stdout).unwrap();
    archives.iter().find(|x| x.from_file_cmd(result))
}

pub async fn get_archive<'a>(
    file_path: &'a str,
    archives: &'a Vec<Box<dyn Archive>>,
) -> Result<&'a Box<dyn Archive>, RaError> {
    match archives.iter().find(|x| x.from_filename(file_path)) {
        Some(x) => Ok(x),
        None => detect_by_file_cmd(file_path, archives)
            .await
            .ok_or(RaError::UnSupportedFormat {
                path: file_path.to_string(),
            }),
    }
}
