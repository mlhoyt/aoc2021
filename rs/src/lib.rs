use std::error::Error;
use std::io::{BufRead, BufReader};

pub mod grid2d;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AocError {
    details: String,
}

impl AocError {
    pub fn new(s: &str) -> Self {
        Self {
            details: s.to_string(),
        }
    }
}

impl std::fmt::Display for AocError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.details)
    }
}

impl std::error::Error for AocError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<std::io::Error> for AocError {
    fn from(err: std::io::Error) -> Self {
        #[allow(deprecated)]
        Self::new(err.description())
    }
}

impl From<peg::error::ParseError<peg::str::LineCol>> for AocError {
    fn from(err: peg::error::ParseError<peg::str::LineCol>) -> Self {
        #[allow(deprecated)]
        Self::new(err.description())
    }
}

pub fn read_file(name: &str) -> Result<String, AocError> {
    let file = std::fs::File::open(name)?;
    let mut file = BufReader::new(file);

    let mut input = String::new();
    while file.read_line(&mut input)? > 0 {}

    Ok(input)
}
