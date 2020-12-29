use std::vec::Vec;
use std::option::Option;
use std::collections::HashSet;
use strum::IntoEnumIterator;

use crate::{Board, Color, Position, Tier, Stack};

type Mask = HashSet<Position>;

trait Pattern {
    fn fit(&self, board: &Board) -> Vec<Mask> {
        let mut fits = Vec::new();
        for position in Position::iter() {
            if let Some(mask) = self.fit_at(&position, &board) {
                fits.push(mask);
            }
        }
        fits
    }

    fn fit_at(&self, positon: &Position, board: &Board) -> Option<Mask>;
}

impl Pattern for Color {
    fn fit_at(&self, position: &Position, board: &Board) -> Option<Mask> {
        if let Some(stack) = board.slots.get(&position) {
            if stack.color == *self {
                return Some([*position].iter().cloned().collect());
            }
        }
        None
    }
}

impl Pattern for Tier {
    fn fit_at(&self, position: &Position, board: &Board) -> Option<Mask> {
        if let Some(stack) = board.slots.get(&position) {
            if stack.tier == *self {
                return Some([*position].iter().cloned().collect());
            }
        }
        None
    }
}

impl Pattern for Stack {
    fn fit_at(&self, position: &Position, board: &Board) -> Option<Mask> {
        if let Some(stack) = board.slots.get(&position) {
            if stack == self {
                return Some([*position].iter().cloned().collect());
            }
        }
        None
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

    #[test]
    fn stack_fit() -> Result<(), Error> {
        let board = Board::try_from("g2j2 g2k3 y3i1 r1i2 g2k4 y3k2")?;
        assert_eq!(Stack::try_from("g2")?.fit(&board), vec![
            [Position::j2].iter().cloned().collect(),
            [Position::k3].iter().cloned().collect(),
            [Position::k4].iter().cloned().collect(),
        ]);
        assert_eq!(Stack::try_from("y3")?.fit(&board), vec![
            [Position::i1].iter().cloned().collect(),
            [Position::k2].iter().cloned().collect(),
        ]);
        assert_eq!(Stack::try_from("r1")?.fit(&board), vec![
            [Position::i2].iter().cloned().collect(),
        ]);
        assert_eq!(Stack::try_from("b2")?.fit(&board).len(), 0);
        Ok(())
    }
}
