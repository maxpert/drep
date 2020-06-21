use std::fmt;
use std::io;

#[derive(Debug)]
pub enum FiltersLoadError {
    IoError(io::Error),
    ParseError(regex::Error),
}

impl From<io::Error> for FiltersLoadError {
    fn from(e: io::Error) -> Self {
        FiltersLoadError::IoError(e)
    }
}

impl From<regex::Error> for FiltersLoadError {
    fn from(e: regex::Error) -> Self {
        FiltersLoadError::ParseError(e)
    }
}

impl fmt::Display for FiltersLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            FiltersLoadError::IoError(e) => write!(f, "Loading error: {}", e),
            FiltersLoadError::ParseError(e) => write!(f, "Loading error: {}", e)
        }
    }
}