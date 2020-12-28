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
            match notation.chars().next() {
                Some('r') => Ok(Color::Red),
                Some('g') => Ok(Color::Green),
                Some('b') => Ok(Color::Blue),
                Some('y') => Ok(Color::Yellow),
                _ => Err(NotationError),
            }
        }
    }
}

#[derive(Debug)]
enum Tier {
    First,
    Second,
    Third,
    Fourth,
}

impl TryFrom<&str> for Tier {
    type Error = NotationError;

    fn try_from(notation: &str) -> Result<Self, Self::Error> {
        if notation.len() != 1 {
            Err(NotationError)
        } else {
            match notation.chars().next() {
                Some('1') => Ok(Tier::First),
                Some('2') => Ok(Tier::Second),
                Some('3') => Ok(Tier::Third),
                Some('4') => Ok(Tier::Fourth),
                _ => Err(NotationError),
            }
        }
    }
}

#[derive(Debug)]
struct Stack {
    color: Color,
    tier: Tier,
    // TODO position: Option<>,
}

impl TryFrom<&str> for Stack {
    type Error = NotationError;

    fn try_from(notation: &str) -> Result<Self, Self::Error> {
        if notation.len() != 2 {
            Err(NotationError)
        } else {
            Ok(Stack {
                color: Color::try_from(&notation[0..1])?,
                tier: Tier::try_from(&notation[1..2])?,
            })
        }
    }
}

#[derive(Debug)]
struct Board {
    stacks: [[Option<Stack>; 4]; 4],
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
        assert!(Color::try_from("R").is_err());
        assert!(Color::try_from("G").is_err());
        assert!(Color::try_from("B").is_err());
        assert!(Color::try_from("Y").is_err());
        assert!(Color::try_from("x").is_err());
        assert!(Color::try_from("sdfsf").is_err());
        assert!(Color::try_from("red").is_err());
        Ok(())
    }

    #[test]
    fn tier_notation() -> Result<(), NotationError> {
        Tier::try_from("1")?;
        Tier::try_from("2")?;
        Tier::try_from("3")?;
        Tier::try_from("4")?;
        assert!(Tier::try_from("5").is_err());
        assert!(Tier::try_from("0").is_err());
        assert!(Tier::try_from("sdfsf").is_err());
        Ok(())
    }

    #[test]
    fn stack_notation() -> Result<(), NotationError> {
        Stack::try_from("r3")?;
        Stack::try_from("g1")?;
        assert!(Stack::try_from("r0").is_err());
        assert!(Stack::try_from("e3").is_err());
        assert!(Stack::try_from("sdfsf").is_err());
        Ok(())
    }
}

