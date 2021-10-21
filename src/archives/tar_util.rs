use tokio::process::Command;

pub enum Tar {
    Bz2,
    Gz,
    Xz,
    Zip,
    Lzma,
    Lzop,
}

impl Tar {
    pub fn archive_option(&self) -> &str {
        match self {
            Tar::Bz2 => "--bzip2",
            Tar::Gz => "--gzip",
            Tar::Xz => "--xz",
            Tar::Zip => "--zip",
            Tar::Lzma => "--lzma",
            Tar::Lzop => "--lzop",
        }
    }
}

pub fn list(src_path: &str) -> Command {
    let mut cmd = Command::new("tar");
    cmd.arg("-tvf").arg(src_path);
    cmd
}

pub fn compress(src_paths: &Vec<&str>, dst_path: &str, tar_type: Tar) -> Command {
    let mut cmd = Command::new("tar");
    cmd.arg("-cvf")
        .arg(tar_type.archive_option())
        .arg(dst_path)
        .args(src_paths);
    cmd
}

pub fn uncompress(src_path: &str, dst_path: &str) -> Command {
    let mut cmd = Command::new("tar");
    cmd.arg("-xvf").arg(src_path).arg("-C").arg(dst_path);
    cmd
}
