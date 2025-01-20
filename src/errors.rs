use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum FileReaderError {
    IoError(std::io::Error),
    JsonParseError(String),
    EmptyJsonArray,
    InvalidJsonStructure(String),
    InconsistentArrayLengths,
    UnsupportedJsonStructure,
}

impl fmt::Display for FileReaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileReaderError::IoError(err) => write!(f, "I/O Error: {}", err),
            FileReaderError::JsonParseError(err) => write!(f, "JSON Parse Error: {}", err),
            FileReaderError::EmptyJsonArray => write!(f, "JSON array is empty"),
            FileReaderError::InvalidJsonStructure(msg) => write!(f, "Invalid JSON structure: {}", msg),
            FileReaderError::InconsistentArrayLengths => {
                write!(f, "Inconsistent array lengths in JSON")
            }
            FileReaderError::UnsupportedJsonStructure => write!(f, "Unsupported JSON structure"),
        }
    }
}

impl Error for FileReaderError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FileReaderError::IoError(err) => Some(err),
            _ => None,
        }
    }
}
