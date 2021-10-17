use std::error::Error;

pub struct Helpers {}

impl Helpers {
    pub fn to_string(bin: &Vec<u8>) -> Result<String, Box<dyn Error>> {
        let a = std::str::from_utf8(bin)?;
        Ok(a.to_string())
    }
}
