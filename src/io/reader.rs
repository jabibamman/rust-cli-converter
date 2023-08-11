use std::path::Path;
use std::fs::File;
use std::io::{Read, Error as IoError};

pub struct FileReader {}

impl FileReader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn read_file_to_string(&self, path: &Path) -> Result<String, IoError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }
}