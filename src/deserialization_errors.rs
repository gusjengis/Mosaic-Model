use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DeserializeError {
    MissingField(&'static str),
    ParseError(&'static str),
    InvalidFormat(&'static str),
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeserializeError::MissingField(field) => write!(f, "Missing field: {}", field),
            DeserializeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DeserializeError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl Error for DeserializeError {}
