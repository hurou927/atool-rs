mod archives;

use tokio::process::Command;

use crate::archives::archive::archive_list;

#[tokio::main]
async fn main() {
    let archives = archive_list();

    archives
        .iter()
        .for_each(|x| println!("{}", x.from_filename("hoge.tar.bz2")));

    let output: std::process::Output = Command::new("ls")
        .output()
        .await
        .expect("ls command failed to run");
    println!("status of ls: {:?}", output.status);
    println!("stdout of ls: {:?}", std::str::from_utf8(&output.stdout));
    println!("stderr of ls: {:?}", std::str::from_utf8(&output.stderr));
}
