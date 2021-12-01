use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadFileError {
    details: String,
}

impl ReadFileError {
    fn new(s: &str) -> Self {
        Self {
            details: s.to_string(),
        }
    }
}

impl fmt::Display for ReadFileError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.details)
    }
}

impl Error for ReadFileError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<std::io::Error> for ReadFileError {
    fn from(err: std::io::Error) -> Self {
        #[allow(deprecated)]
        ReadFileError::new(err.description())
    }
}

pub fn read_file(name: &str) -> Result<String, ReadFileError> {
    let file = fs::File::open(name)?;
    let mut file = BufReader::new(file);

    let mut input = String::new();
    while file.read_line(&mut input)? > 0 {}

    Ok(input)
}
