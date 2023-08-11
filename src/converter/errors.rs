use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ConversionError {
    InvalidJson,
    InvalidXml,
    MappingError,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConversionError::InvalidJson => write!(f, "Invalid JSON"),
            ConversionError::InvalidXml => write!(f, "Invalid XML"),
            ConversionError::MappingError => write!(f, "Mapping Error"),
        }
    }
}


impl Error for ConversionError {}
