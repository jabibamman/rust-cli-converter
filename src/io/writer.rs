use std::fs::File;
use std::path::Path;
use std::io::{Error as IoError, Write};
use serde_xml_rs::from_str;
use crate::converter::errors::ConversionError;
use lazy_static::lazy_static;

const INDENT_SIZE: usize = 4;
lazy_static! {
    static ref INDENT: String = " ".repeat(INDENT_SIZE);
}
pub struct FileWriter {}

impl FileWriter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_string_to_file(&self, path: &Path, contents: &str) -> Result<(), IoError> {
        let mut file = File::create(path)?;

        if path.extension().unwrap() == "json" {
            serde_json::to_writer_pretty(&mut file, &contents)?;
        } else {
            file.write_all(self.indent_xml(contents).unwrap().as_ref())?;
        }

        Ok(())
    }

    pub fn indent_xml(&self, contents: &str) -> Result<String, IoError> {
        let mut indent: usize = 0;
        let mut result = String::new();
        let mut chars = contents.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '<' if chars.peek() == Some(&'/') => {
                    indent = indent.saturating_sub(1);
                    result.push_str("\n");
                    result.push_str(&INDENT.repeat(indent));
                    result.push(c);
                }
                '<' if chars.peek().is_some() && chars.peek().unwrap().is_alphanumeric() => {
                    if result.ends_with('>') {
                        result.push_str("\n");
                        result.push_str(&INDENT.repeat(indent));
                    }
                    result.push(c);
                    indent += 1;
                },
                '>' => {
                    result.push(c);
                },
                _ => result.push(c),
            }
        }

        self.validate_xml_with_serde(&result)?;

        Ok(result)
    }

    pub fn validate_xml_with_serde(&self, xml: &str) -> Result<(), IoError> {
        match from_str::<serde_json::Value>(xml) {
            Ok(_) => Ok(()),
            Err(_) => Err(IoError::new(std::io::ErrorKind::InvalidData, ConversionError::InvalidXml)),
        }
    }
}