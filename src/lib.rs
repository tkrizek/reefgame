use std::convert::TryFrom;
use std::option::Option;
use crate::error::NotationError;

pub mod error;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl TryFrom<&str> for Color {
    type Error = NotationError;

    fn try_from(notation: &str) -> Result<Self, Self::Error> {
        if notation.len() != 1 {
            Err(NotationError)
        } else {
            match notation.to_lowercase().chars().next() {
                Some('r') => Ok(Color::Red),
                Some('g') => Ok(Color::Green),
                Some('b') => Ok(Color::Blue),
                Some('y') => Ok(Color::Yellow),
                _ => Err(NotationError),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_notation() -> Result<(), NotationError> {
        Color::try_from("r")?;
        Color::try_from("g")?;
        Color::try_from("b")?;
        Color::try_from("y")?;
        Color::try_from("R")?;
        Color::try_from("G")?;
        Color::try_from("B")?;
        Color::try_from("Y")?;
        assert!(Color::try_from("x").is_err());
        assert!(Color::try_from("sdfsf").is_err());
        assert!(Color::try_from("red").is_err());
        Ok(())
    }
}

