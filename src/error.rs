use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotationError,
    MoveError,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotationError => write!(f, "Failed to parse notation"),
            Error::MoveError => write!(f, "Illegal player move"),
        }
    }
}
