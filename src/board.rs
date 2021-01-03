use std::collections::{BTreeSet, HashMap};
use std::convert::TryFrom;
use strum::EnumIter;
use crate::{Error, Stack, pieces::TierLevel};

#[allow(non_camel_case_types)]
#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Position {
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
            _ => Err(Error::InvalidNotation),
        }
    }
}

impl Position {
    fn from_coords(x: u8, y: u8) -> Result<Position, Error> {
        match x {
            1 => {
                match y {
                    1 => Ok(Position::i1),
                    2 => Ok(Position::i2),
                    3 => Ok(Position::i3),
                    4 => Ok(Position::i4),
                    _ => Err(Error::OutOfBounds),
                }
            },
            2 => {
                match y {
                    1 => Ok(Position::j1),
                    2 => Ok(Position::j2),
                    3 => Ok(Position::j3),
                    4 => Ok(Position::j4),
                    _ => Err(Error::OutOfBounds),
                }
            },
            3 => {
                match y {
                    1 => Ok(Position::k1),
                    2 => Ok(Position::k2),
                    3 => Ok(Position::k3),
                    4 => Ok(Position::k4),
                    _ => Err(Error::OutOfBounds),
                }
            },
            4 => {
                match y {
                    1 => Ok(Position::l1),
                    2 => Ok(Position::l2),
                    3 => Ok(Position::l3),
                    4 => Ok(Position::l4),
                    _ => Err(Error::OutOfBounds),
                }
            },
            _ => Err(Error::OutOfBounds),
        }
    }

    fn to_coords(&self) -> (u8, u8) {
        match self {
            Position::i1 => (1, 1),
            Position::i2 => (1, 2),
            Position::i3 => (1, 3),
            Position::i4 => (1, 4),
            Position::j1 => (2, 1),
            Position::j2 => (2, 2),
            Position::j3 => (2, 3),
            Position::j4 => (2, 4),
            Position::k1 => (3, 1),
            Position::k2 => (3, 2),
            Position::k3 => (3, 3),
            Position::k4 => (3, 4),
            Position::l1 => (4, 1),
            Position::l2 => (4, 2),
            Position::l3 => (4, 3),
            Position::l4 => (4, 4),
        }
    }

    pub fn up(&self) -> Option<Position> {
        let (x, y) = self.to_coords();
        Position::from_coords(x, y + 1).ok()
    }

    pub fn down(&self) -> Option<Position> {
        let (x, y) = self.to_coords();
        Position::from_coords(x, y - 1).ok()
    }

    pub fn left(&self) -> Option<Position> {
        let (x, y) = self.to_coords();
        Position::from_coords(x - 1, y).ok()
    }

    pub fn right(&self) -> Option<Position> {
        let (x, y) = self.to_coords();
        Position::from_coords(x + 1, y).ok()
    }

    pub fn upleft(&self) -> Option<Position> {
        let (x, y) = self.to_coords();
        Position::from_coords(x - 1, y + 1).ok()
    }

    pub fn downleft(&self) -> Option<Position> {
        let (x, y) = self.to_coords();
        Position::from_coords(x - 1, y - 1).ok()
    }

    pub fn upright(&self) -> Option<Position> {
        let (x, y) = self.to_coords();
        Position::from_coords(x + 1, y + 1).ok()
    }

    pub fn downright(&self) -> Option<Position> {
        let (x, y) = self.to_coords();
        Position::from_coords(x + 1, y - 1).ok()
    }
}

pub type Mask = BTreeSet<Position>;
pub type MaskSet = BTreeSet<Mask>;

/// Player's board where pieces are placed and stacked.
#[derive(Debug)]
pub struct Board {
    slots: HashMap<Position, Stack>,
}

impl Board {
    /// Create an empty board.
    pub fn new() -> Board {
        Board {
            slots: HashMap::with_capacity(16),
        }
    }

    pub fn get(&self, position: &Position) -> Option<&Stack> {
        self.slots.get(position)
    }

    pub fn interpret(notation: &str) -> Result<(Position, Stack), Error> {
        if notation.len() != 4 {
            return Err(Error::InvalidNotation);
        }
        let position = Position::try_from(&notation[2..])?;
        let stack = Stack::try_from(&notation[0..2])?;
        Ok((position, stack))
    }

    pub fn place(&mut self, position: Position, stack: Stack) -> Result<(), Error> {
        if stack.tier.is_on_top_of(&self.slots.get(&position)) {
            self.slots.insert(position, stack);
            Ok(())
        } else {
            Err(Error::IllegalMove)
        }
    }

    pub fn play(&mut self, notation: &str) -> Result<(), Error> {
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
                    Err(Error::InvalidNotation)
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
    fn position_coords() -> Result<(), Error> {
        assert_eq!(Position::from_coords(1, 1)?, Position::i1);
        assert_eq!(Position::from_coords(2, 1)?, Position::j1);
        assert_eq!(Position::from_coords(3, 1)?, Position::k1);
        assert_eq!(Position::from_coords(4, 1)?, Position::l1);
        assert_eq!(Position::from_coords(2, 2)?, Position::j2);
        assert_eq!(Position::from_coords(2, 3)?, Position::j3);
        assert_eq!(Position::from_coords(2, 4)?, Position::j4);
        assert!(Position::from_coords(5, 1).is_err());
        assert!(Position::from_coords(1, 0).is_err());
        assert_eq!(Position::k2.up(), Some(Position::k3));
        assert_eq!(Position::k4.up(), None);
        assert_eq!(Position::k2.down(), Some(Position::k1));
        assert_eq!(Position::k1.down(), None);
        assert_eq!(Position::k2.left(), Some(Position::j2));
        assert_eq!(Position::i2.left(), None);
        assert_eq!(Position::k2.right(), Some(Position::l2));
        assert_eq!(Position::l4.right(), None);
        assert_eq!(Position::k2.upleft(), Some(Position::j3));
        assert_eq!(Position::i3.upleft(), None);
        assert_eq!(Position::k2.downleft(), Some(Position::j1));
        assert_eq!(Position::l1.downleft(), None);
        assert_eq!(Position::k2.upright(), Some(Position::l3));
        assert_eq!(Position::l3.upright(), None);
        assert_eq!(Position::k2.downright(), Some(Position::l1));
        assert_eq!(Position::j1.downright(), None);
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
