use std::vec::Vec;
use std::collections::HashSet;
use strum::IntoEnumIterator;

use crate::{Board, Color, Position, Tier};

type Mask = HashSet<Position>;

trait Pattern {
    fn fit(&self, board: &Board) -> Vec<Mask>;
}

impl Pattern for Color {
    fn fit(&self, board: &Board) -> Vec<Mask> {
        let mut matches = Vec::new();
        for position in Position::iter() {
            if let Some(stack) = board.slots.get(&position) {
                if stack.color == *self {
                    matches.push([position].iter().cloned().collect());
                }
            }
        }
        matches
    }
}

impl Pattern for Tier {
    fn fit(&self, board: &Board) -> Vec<Mask> {
        let mut matches = Vec::new();
        for position in Position::iter() {
            if let Some(stack) = board.slots.get(&position) {
                if stack.tier == *self {
                    matches.push([position].iter().cloned().collect());
                }
            }
        }
        matches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use std::convert::TryFrom;

    #[test]
    fn color_fit() -> Result<(), Error> {
        let board = Board::try_from("r3i2 r1i3 g2j4 y1j1 r4k1")?;
        assert_eq!(Color::Red.fit(&board), vec![
            [Position::i2].iter().cloned().collect(),
            [Position::i3].iter().cloned().collect(),
            [Position::k1].iter().cloned().collect(),
        ]);
        assert_eq!(Color::Green.fit(&board), vec![
            [Position::j4].iter().cloned().collect(),
        ]);
        assert_eq!(Color::Yellow.fit(&board), vec![
            [Position::j1].iter().cloned().collect(),
        ]);
        assert_eq!(Color::Blue.fit(&board).len(), 0);
        Ok(())
    }

    #[test]
    fn tier_fit() -> Result<(), Error> {
        let board = Board::try_from("r3i2 r1i3 g2j4 y1j1 r4k1")?;
        assert_eq!(Tier::First.fit(&board), vec![
            [Position::i3].iter().cloned().collect(),
            [Position::j1].iter().cloned().collect(),
        ]);
        assert_eq!(Tier::Second.fit(&board), vec![
            [Position::j4].iter().cloned().collect(),
        ]);
        assert_eq!(Tier::Third.fit(&board), vec![
            [Position::i2].iter().cloned().collect(),
        ]);
        assert_eq!(Tier::Fourth.fit(&board), vec![
            [Position::k1].iter().cloned().collect(),
        ]);
        Ok(())
    }
}
