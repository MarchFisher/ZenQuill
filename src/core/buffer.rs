use std::error::Error;
use std::fs::read_to_string;

/// buffer
#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let contents = read_to_string(file_name)?;

        let mut lines = Vec::new();
        for line in contents.lines() {
            lines.push(line.to_string());
        }

        Ok(Self { lines })
    }
}