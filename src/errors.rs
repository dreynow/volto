use std::fmt;
use std::error::Error;
use thiserror::Error;

#[derive(Debug)]
pub enum FileReaderError {
    #[error("I/O Error: {0}")]
    IoError(#[from] io::Error),

    #[error("JSON Parse Error: {0}")]
    JsonParseError(String),

    #[error("JSON array is empty")]
    EmptyJsonArray,

    #[error("Invalid JSON structure: {0}")]
    InvalidJsonStructure(String),

    #[error("Inconsistent array lengths in JSON")]
    InconsistentArrayLengths,

    #[error("Unsupported JSON structure")]
    UnsupportedJsonStructure,

    #[error("Excel Error: {0}")]
    ExcelError(String),
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
            FileReaderError::ExcelError(msg) => write!(f, "Excel Error: {}", msg),
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
