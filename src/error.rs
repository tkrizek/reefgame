use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidNotation,
    IllegalMove,
    OutOfBounds,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidNotation => write!(f, "Failed to parse notation"),
            Error::IllegalMove => write!(f, "Illegal player move"),
            Error::OutOfBounds => write!(f, "Index out of bounds"),
        }
    }
}
