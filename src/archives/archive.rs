use tokio::process::Command;

use crate::{
    archives::{tarbz2::TarBz2, targz::TarGz, tarlzma::TarLzma, tarlzop::TarLzop, tarxz::TarXz},
    ra_error::RaError,
};

pub struct LsParam<'a> {
    pub src_path: &'a str,
}

pub struct PackParam<'a> {
    pub src_paths: Vec<&'a str>,
    pub dst_path: &'a str,
}
pub struct UnPackParam<'a> {
    pub src_path: &'a str,
    pub dst_path: &'a str,
}

#[derive(Clone, Copy, Debug)]
pub enum ArchiveType {
    TarBz2,
    TarGz,
    TarXz,
    TarLzma,
    TarLzop,
}

impl ArchiveType {
    pub fn to_string(&self) -> &str {
        match self {
            &ArchiveType::TarBz2 => "tar+bz2",
            &ArchiveType::TarGz => "tar+gz",
            &ArchiveType::TarXz => "tar+xz",
            &ArchiveType::TarLzop => "tar+lzop",
            &ArchiveType::TarLzma => "tar+lzma",
        }
    }
    pub fn from_string(&self, src: &str) -> Option<ArchiveType> {
        match src {
            "tar+bz2" => Some(ArchiveType::TarBz2),
            "tar+gz" => Some(ArchiveType::TarGz),
            "tar+xz" => Some(ArchiveType::TarXz),
            "tar+lzop" => Some(ArchiveType::TarLzop),
            "tar+lzma" => Some(ArchiveType::TarLzma),
            _ => None,
        }
    }
}

pub trait Archive {
    fn get_type(&self) -> ArchiveType;
    fn from_filename(&self, filename: &str) -> bool;
    fn from_file_cmd(&self, stdout: &str) -> bool;
    fn ls(&self, src: &LsParam) -> Command;
    fn pack(&self, src: &PackParam) -> Command;
    fn unpack(&self, src: &UnPackParam) -> Command;
}

pub fn archive_list() -> Vec<Box<dyn Archive>> {
    vec![
        Box::new(TarBz2::new()),
        Box::new(TarGz::new()),
        Box::new(TarXz::new()),
        Box::new(TarLzma::new()),
        Box::new(TarLzop::new()),
    ]
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
