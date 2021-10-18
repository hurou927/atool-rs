use std::process::Command;

pub struct TestUtil {}

impl TestUtil {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TestUtil {}
    }
    #[allow(dead_code)]
    pub fn resource_path(name: &str) -> String {
        format!("tests/resource/{}", name)
    }
    #[allow(dead_code)]
    pub fn file_cmd_result(file_path: &str) -> String {
        let output = Command::new("file")
            .arg("-z")
            .arg("-L")
            .arg("-b")
            .arg(file_path)
            .output()
            .expect("failed to execute process");
        std::str::from_utf8(&output.stdout).unwrap().to_string()
    }
}
