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
            if let Some(mask) = self.fit_at_90deg(&position, &board) {
                fits.push(mask);
            }
            if let Some(mask) = self.fit_at_180deg(&position, &board) {
                fits.push(mask);
            }
            if let Some(mask) = self.fit_at_270deg(&position, &board) {
                fits.push(mask);
            }
        }
        fits
    }

    fn fit_at(&self, positon: &Position, board: &Board) -> Option<Mask>;
    fn fit_at_90deg(&self, _position: &Position, _board: &Board) -> Option<Mask> {
        None
    }
    fn fit_at_180deg(&self, _position: &Position, _board: &Board) -> Option<Mask> {
        None
    }
    fn fit_at_270deg(&self, _position: &Position, _board: &Board) -> Option<Mask> {
        None
    }
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

struct AdjacentColors {
    first: Color,
    second: Color,
}

impl AdjacentColors {
    fn new(first: Color, second: Color) -> AdjacentColors {
        AdjacentColors {
            first: first,
            second: second,
        }
    }

    fn fit_colors(&self, pos1: &Position, pos2: &Position, board: &Board) -> Option<Mask> {
        let stack1 = board.slots.get(&pos1);
        let stack2 = board.slots.get(&pos2);
        if let (Some(stack1), Some(stack2)) = (stack1, stack2) {
            let fit = stack1.color == self.first && stack2.color == self.second;
            let fit_inv = stack1.color == self.second && stack2.color == self.first;
            if fit || fit_inv {
                return Some([*pos1, *pos2].iter().cloned().collect());
            }
        }
        None
    }
}

impl Pattern for AdjacentColors {
    fn fit_at(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.right().ok() {
            self.fit_colors(pos1, &pos2, &board)
        } else {
            None
        }
    }

    fn fit_at_90deg(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.up().ok() {
            self.fit_colors(pos1, &pos2, &board)
        } else {
            None
        }
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

    #[test]
    fn adjacent_colors_fit() -> Result<(), Error> {
        //    G1 Y1 Y2
        // R2 B3
        // B2 G4 B3 R2
        // G1 B3 Y2
        let board = Board::try_from("g1i1 b2i2 r2i3 b3j1 g4j2 b3j3 g1j4 y2k1 b3k2 y1k4 r2l2 y2l4")?;
        let gb = AdjacentColors::new(Color::Green, Color::Blue);
        assert_eq!(gb.fit(&board), vec![
            [Position::i1, Position::j1].iter().cloned().collect(),
            [Position::i1, Position::i2].iter().cloned().collect(),
            [Position::i2, Position::j2].iter().cloned().collect(),
            [Position::j1, Position::j2].iter().cloned().collect(),
            [Position::j2, Position::k2].iter().cloned().collect(),
            [Position::j2, Position::j3].iter().cloned().collect(),
            [Position::j3, Position::j4].iter().cloned().collect(),
        ]);
        Ok(())
    }
}
