use crate::error::Error;
use std::collections::HashMap;
use std::convert::TryFrom;

pub mod error;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl TryFrom<&str> for Color {
    type Error = Error;

    fn try_from(notation: &str) -> Result<Self, Self::Error> {
        if notation.len() != 1 {
            Err(Error::NotationError)
        } else {
            match notation.chars().next() {
                Some('r') => Ok(Color::Red),
                Some('g') => Ok(Color::Green),
                Some('b') => Ok(Color::Blue),
                Some('y') => Ok(Color::Yellow),
                _ => Err(Error::NotationError),
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tier {
    First,
    Second,
    Third,
    Fourth,
}

impl TryFrom<&str> for Tier {
    type Error = Error;

    fn try_from(notation: &str) -> Result<Self, Self::Error> {
        if notation.len() != 1 {
            Err(Error::NotationError)
        } else {
            match notation.chars().next() {
                Some('1') => Ok(Tier::First),
                Some('2') => Ok(Tier::Second),
                Some('3') => Ok(Tier::Third),
                Some('4') => Ok(Tier::Fourth),
                _ => Err(Error::NotationError),
            }
        }
    }
}

trait TierLevel {
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
struct Stack {
    color: Color,
    tier: Tier,
}

impl TryFrom<&str> for Stack {
    type Error = Error;

    fn try_from(notation: &str) -> Result<Self, Self::Error> {
        if notation.len() != 2 {
            Err(Error::NotationError)
        } else {
            Ok(Stack {
                color: Color::try_from(&notation[0..1])?,
                tier: Tier::try_from(&notation[1..2])?,
            })
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Position {
    i1,
    i2,
    i3,
    i4,
    j1,
    j2,
    j3,
    j4,
    k1,
    k2,
    k3,
    k4,
    l1,
    l2,
    l3,
    l4,
}

impl TryFrom<&str> for Position {
    type Error = Error;

    fn try_from(notation: &str) -> Result<Self, Self::Error> {
        match notation {
            "i1" => Ok(Position::i1),
            "i2" => Ok(Position::i2),
            "i3" => Ok(Position::i3),
            "i4" => Ok(Position::i4),
            "j1" => Ok(Position::j1),
            "j2" => Ok(Position::j2),
            "j3" => Ok(Position::j3),
            "j4" => Ok(Position::j4),
            "k1" => Ok(Position::k1),
            "k2" => Ok(Position::k2),
            "k3" => Ok(Position::k3),
            "k4" => Ok(Position::k4),
            "l1" => Ok(Position::l1),
            "l2" => Ok(Position::l2),
            "l3" => Ok(Position::l3),
            "l4" => Ok(Position::l4),
            _ => Err(Error::NotationError),
        }
    }
}

#[derive(Debug)]
struct Board {
    slots: HashMap<Position, Stack>,
}

impl Board {
    fn new() -> Board {
        Board {
            slots: HashMap::with_capacity(16),
        }
    }

    fn interpret(notation: &str) -> Result<(Position, Stack), Error> {
        if notation.len() != 4 {
            return Err(Error::NotationError);
        }
        let position = Position::try_from(&notation[2..])?;
        let stack = Stack::try_from(&notation[0..2])?;
        Ok((position, stack))
    }

    fn place(&mut self, position: Position, stack: Stack) -> Result<(), Error> {
        if stack.tier.is_on_top_of(&self.slots.get(&position)) {
            self.slots.insert(position, stack);
            Ok(())
        } else {
            Err(Error::MoveError)
        }
    }

    fn play(&mut self, notation: &str) -> Result<(), Error> {
        let (position, stack) = Board::interpret(notation)?;
        self.place(position, stack)
    }
}

impl TryFrom<&str> for Board {
    type Error = Error;

    fn try_from(notations: &str) -> Result<Self, Self::Error> {
        let mut board = Board::new();
        notations
            .split(" ")
            .try_fold(&mut board, |board, notation: &str| {
                let (position, stack) = Board::interpret(notation)?;
                if board.slots.contains_key(&position) {
                    Err(Error::NotationError)
                } else {
                    board.slots.insert(position, stack);
                    Ok(board)
                }
            })?;
        Ok(board)
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

    #[test]
    fn position_notation() -> Result<(), Error> {
        Position::try_from("i1")?;
        Position::try_from("i2")?;
        Position::try_from("i3")?;
        Position::try_from("i4")?;
        Position::try_from("j1")?;
        Position::try_from("j2")?;
        Position::try_from("j3")?;
        Position::try_from("j4")?;
        Position::try_from("k1")?;
        Position::try_from("k2")?;
        Position::try_from("k3")?;
        Position::try_from("k4")?;
        Position::try_from("l1")?;
        Position::try_from("l2")?;
        Position::try_from("l3")?;
        Position::try_from("l4")?;
        assert!(Position::try_from("r3").is_err());
        assert!(Position::try_from("I2").is_err());
        assert!(Position::try_from("dafd").is_err());
        assert!(Position::try_from("x").is_err());
        Ok(())
    }

    #[test]
    fn board_interpret() -> Result<(), Error> {
        Board::interpret("r3i1")?;
        Board::interpret("y4k3")?;
        Board::interpret("r3i1")?;
        assert!(Board::interpret("r3i1 ").is_err());
        assert!(Board::interpret("x").is_err());
        Ok(())
    }

    #[test]
    fn board_notation() -> Result<(), Error> {
        Board::try_from("r3i1")?;
        Board::try_from("r3i1 y4k3")?;
        assert!(Board::try_from("r3i1 r3i1").is_err());
        assert!(Board::try_from("r3i1 y4k3 ").is_err());
        Ok(())
    }

    #[test]
    fn board_place() -> Result<(), Error> {
        let mut board = Board::try_from("r1j3 g1k4 b1j2 y1k2")?;
        board.play("y1l4")?;
        board.play("r2j3")?;
        board.play("r3j3")?;
        board.play("b4j3")?;
        assert!(board.play("g3l4").is_err());
        Ok(())
    }
}
