use std::io;

#[derive(Debug)]
pub enum LoadError {
    IoError(io::Error),
    ParseError(regex::Error)
}

impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> Self {
        LoadError::IoError(e)
    }
}

impl From<regex::Error> for LoadError {
    fn from(e: regex::Error) -> Self {
        LoadError::ParseError(e)
    }
}