use std::convert::TryFrom;
use crate::Error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl TryFrom<&str> for Color {
    type Error = Error;

    fn try_from(notation: &str) -> Result<Self, Self::Error> {
        if notation.len() != 1 {
            Err(Error::InvalidNotation)
        } else {
            match notation.chars().next() {
                Some('r') => Ok(Color::Red),
                Some('g') => Ok(Color::Green),
                Some('b') => Ok(Color::Blue),
                Some('y') => Ok(Color::Yellow),
                _ => Err(Error::InvalidNotation),
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Tier {
    First,
    Second,
    Third,
    Fourth,
}

impl TryFrom<&str> for Tier {
    type Error = Error;

    fn try_from(notation: &str) -> Result<Self, Self::Error> {
        if notation.len() != 1 {
            Err(Error::InvalidNotation)
        } else {
            match notation.chars().next() {
                Some('1') => Ok(Tier::First),
                Some('2') => Ok(Tier::Second),
                Some('3') => Ok(Tier::Third),
                Some('4') => Ok(Tier::Fourth),
                _ => Err(Error::InvalidNotation),
            }
        }
    }
}

pub trait TierLevel {
    fn level(&self) -> u8;

    fn is_on_top_of(&self, previous: &impl TierLevel) -> bool {
        self.level() == previous.level() + 1
    }
}

impl TierLevel for Tier {
    fn level(&self) -> u8 {
        match self {
            Tier::First => 1,
            Tier::Second => 2,
            Tier::Third => 3,
            Tier::Fourth => 4,
        }
    }
}

impl TierLevel for Stack {
    fn level(&self) -> u8 {
        self.tier.level()
    }
}

impl TierLevel for Option<&Stack> {
    fn level(&self) -> u8 {
        match self {
            Some(stack) => stack.tier.level(),
            None => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Stack {
    pub color: Color,
    pub tier: Tier,
}

impl TryFrom<&str> for Stack {
    type Error = Error;

    fn try_from(notation: &str) -> Result<Self, Self::Error> {
        if notation.len() != 2 {
            Err(Error::InvalidNotation)
        } else {
            Ok(Stack {
                color: Color::try_from(&notation[0..1])?,
                tier: Tier::try_from(&notation[1..2])?,
            })
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_notation() -> Result<(), Error> {
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
    fn tier_notation() -> Result<(), Error> {
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
    fn tier_level_is_on_top_of() {
        assert!(Tier::First.is_on_top_of(&None));
        assert!(Tier::Second.is_on_top_of(&Tier::First));
        assert!(Tier::Third.is_on_top_of(&Tier::Second));
        assert!(Tier::Fourth.is_on_top_of(&Tier::Third));
        assert_eq!(Tier::First.is_on_top_of(&Tier::First), false);
        assert_eq!(Tier::Third.is_on_top_of(&Tier::First), false);
        assert!(Stack::try_from("r4")
            .unwrap()
            .is_on_top_of(&Stack::try_from("g3").unwrap()));
    }

    #[test]
    fn stack_notation() -> Result<(), Error> {
        Stack::try_from("r3")?;
        Stack::try_from("g1")?;
        assert!(Stack::try_from("r0").is_err());
        assert!(Stack::try_from("e3").is_err());
        assert!(Stack::try_from("sdfsf").is_err());
        assert!(Stack::try_from("x").is_err());
        Ok(())
    }
}
