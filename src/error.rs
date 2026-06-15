use std::fmt;

use crate::io;
#[derive(Debug)]
pub enum MoveError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
    InvalidRow,
    InvalidCol,
    AlreadyOccupied,
}

impl From<io::Error> for MoveError {
    fn from(e: io::Error) -> Self {
        MoveError::Io(e)
    }
}
impl From<std::num::ParseIntError> for MoveError {
    fn from(e: std::num::ParseIntError) -> Self {
        MoveError::Parse(e)
    }
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveError::Io(e) => write!(f, "I/O error: {}", e),
            MoveError::Parse(e) => write!(f, "Could not parse number: {}", e),
            MoveError::InvalidRow => write!(f, "Row must be between 1 and 3"),
            MoveError::InvalidCol => write!(f, "Column must be between 1 and 3"),
            MoveError::AlreadyOccupied => write!(f, "That field is already occupied"),
        }
    }
}
