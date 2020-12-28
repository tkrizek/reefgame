use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct NotationError;

impl Error for NotationError { }

impl fmt::Display for NotationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid notation")
    }
}
