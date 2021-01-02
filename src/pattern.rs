use std::vec::Vec;
use std::option::Option;
use std::collections::BTreeSet;
use strum::IntoEnumIterator;

use crate::{Board, Color, Position, Tier, Stack};

type Mask = BTreeSet<Position>;

trait Pattern {
    fn fit(&self, board: &Board) -> BTreeSet<Mask> {
        let mut fits = BTreeSet::new();
        for position in Position::iter() {
            if let Some(mask) = self.fit_at(&position, &board) {
                fits.insert(mask);
            }
            if let Some(mask) = self.fit_at_90deg(&position, &board) {
                fits.insert(mask);
            }
            if let Some(mask) = self.fit_at_180deg(&position, &board) {
                fits.insert(mask);
            }
            if let Some(mask) = self.fit_at_270deg(&position, &board) {
                fits.insert(mask);
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
                return Some(btreeset!{*position});
            }
        }
        None
    }
}

impl Pattern for Tier {
    fn fit_at(&self, position: &Position, board: &Board) -> Option<Mask> {
        if let Some(stack) = board.slots.get(&position) {
            if stack.tier == *self {
                return Some(btreeset!{*position});
            }
        }
        None
    }
}

impl Pattern for Stack {
    fn fit_at(&self, position: &Position, board: &Board) -> Option<Mask> {
        if let Some(stack) = board.slots.get(&position) {
            if stack == self {
                return Some(btreeset!{*position});
            }
        }
        None
    }
}

struct AdjacentColors(Color, Color);

impl AdjacentColors {
    fn fit_colors(&self, pos1: &Position, pos2: &Position, board: &Board) -> Option<Mask> {
        let stack1 = board.slots.get(&pos1);
        let stack2 = board.slots.get(&pos2);
        if let (Some(stack1), Some(stack2)) = (stack1, stack2) {
            let fit = stack1.color == self.0 && stack2.color == self.1;
            let fit_inv = stack1.color == self.1 && stack2.color == self.0;
            if fit || fit_inv {
                return Some(btreeset!{*pos1, *pos2});
            }
        }
        None
    }
}

impl Pattern for AdjacentColors {
    fn fit_at(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.right() {
            self.fit_colors(pos1, &pos2, &board)
        } else {
            None
        }
    }

    fn fit_at_90deg(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.up() {
            self.fit_colors(pos1, &pos2, &board)
        } else {
            None
        }
    }
}

trait Shape {
    fn fit_mask(&self, mask: Mask, board: &Board) -> Option<Mask>;
}

impl Shape for Color {
    fn fit_mask(&self, mask: Mask, board: &Board) -> Option<Mask> {
        for position in mask.iter() {
            if self.fit_at(position, board) == None {
                return None;
            }
        }
        Some(mask)
    }
}

struct Diagonal(Color);

impl Pattern for Diagonal {
    fn fit_at(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.upright() {
            if let Some(pos3) = pos2.upright() {
                let mask = btreeset!{*pos1, pos2, pos3};
                return self.0.fit_mask(mask, board);
            }
        }
        None
    }

    fn fit_at_90deg(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.downright() {
            if let Some(pos3) = pos2.downright() {
                let mask = btreeset!{*pos1, pos2, pos3};
                return self.0.fit_mask(mask, board);
            }
        }
        None
    }
}

struct Line(Color);

impl Pattern for Line {
    fn fit_at(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.right() {
            if let Some(pos3) = pos2.right() {
                let mask = btreeset!{*pos1, pos2, pos3};
                return self.0.fit_mask(mask, board);
            }
        }
        None
    }

    fn fit_at_90deg(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.up() {
            if let Some(pos3) = pos2.up() {
                let mask = btreeset!{*pos1, pos2, pos3};
                return self.0.fit_mask(mask, board);
            }
        }
        None
    }
}

struct Corner(Color);

impl Pattern for Corner {
    fn fit_at(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.up() {
            if let Some(pos3) = pos2.right() {
                let mask = btreeset!{*pos1, pos2, pos3};
                return self.0.fit_mask(mask, board);
            }
        }
        None
    }

    fn fit_at_90deg(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.left() {
            if let Some(pos3) = pos2.up() {
                let mask = btreeset!{*pos1, pos2, pos3};
                return self.0.fit_mask(mask, board);
            }
        }
        None
    }

    fn fit_at_180deg(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.down() {
            if let Some(pos3) = pos2.left() {
                let mask = btreeset!{*pos1, pos2, pos3};
                return self.0.fit_mask(mask, board);
            }
        }
        None
    }

    fn fit_at_270deg(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.right() {
            if let Some(pos3) = pos2.down() {
                let mask = btreeset!{*pos1, pos2, pos3};
                return self.0.fit_mask(mask, board);
            }
        }
        None
    }
}

struct Square(Color);

impl Pattern for Square {
    fn fit_at(&self, pos1: &Position, board: &Board) -> Option<Mask> {
        if let Some(pos2) = pos1.up() {
            if let Some(pos3) = pos2.right() {
                if let Some(pos4) = pos1.right() {
                    let mask = btreeset!{*pos1, pos2, pos3, pos4};
                    return self.0.fit_mask(mask, board);
                }
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
        assert_eq!(Color::Red.fit(&board), btreeset!{
            btreeset!{Position::i2},
            btreeset!{Position::i3},
            btreeset!{Position::k1},
        });
        assert_eq!(Color::Green.fit(&board), btreeset!{
            btreeset!{Position::j4},
        });
        assert_eq!(Color::Yellow.fit(&board), btreeset!{
            btreeset!{Position::j1},
        });
        assert_eq!(Color::Blue.fit(&board).len(), 0);
        Ok(())
    }

    #[test]
    fn tier_fit() -> Result<(), Error> {
        let board = Board::try_from("r3i2 r1i3 g2j4 y1j1 r4k1")?;
        assert_eq!(Tier::First.fit(&board), btreeset!{
            btreeset!{Position::i3},
            btreeset!{Position::j1},
        });
        assert_eq!(Tier::Second.fit(&board), btreeset!{
            btreeset!{Position::j4},
        });
        assert_eq!(Tier::Third.fit(&board), btreeset!{
            btreeset!{Position::i2},
        });
        assert_eq!(Tier::Fourth.fit(&board), btreeset!{
            btreeset!{Position::k1},
        });
        Ok(())
    }

    #[test]
    fn stack_fit() -> Result<(), Error> {
        let board = Board::try_from("g2j2 g2k3 y3i1 r1i2 g2k4 y3k2")?;
        assert_eq!(Stack::try_from("g2")?.fit(&board), btreeset!{
            btreeset!{Position::j2},
            btreeset!{Position::k3},
            btreeset!{Position::k4},
        });
        assert_eq!(Stack::try_from("y3")?.fit(&board), btreeset!{
            btreeset!{Position::i1},
            btreeset!{Position::k2},
        });
        assert_eq!(Stack::try_from("r1")?.fit(&board), btreeset!{
            btreeset!{Position::i2},
        });
        assert_eq!(Stack::try_from("b2")?.fit(&board).len(), 0);
        Ok(())
    }

    #[test]
    fn adjacent_colors_fit() -> Result<(), Error> {
        //    g1 y1 y2
        // r2 b3
        // b2 g4 b3 r2
        // g1 b3 y2
        let board = Board::try_from("g1i1 b2i2 r2i3 b3j1 g4j2 b3j3 g1j4 y2k1 b3k2 y1k4 r2l2 y2l4")?;
        let gb = AdjacentColors(Color::Green, Color::Blue);
        assert_eq!(gb.fit(&board), btreeset!{
            btreeset!{Position::i1, Position::j1},
            btreeset!{Position::i1, Position::i2},
            btreeset!{Position::i2, Position::j2},
            btreeset!{Position::j1, Position::j2},
            btreeset!{Position::j2, Position::k2},
            btreeset!{Position::j2, Position::j3},
            btreeset!{Position::j3, Position::j4},
        });
        Ok(())
    }

    #[test]
    fn diagonal_fit() -> Result<(), Error> {
        //    g1 r1
        // r2 r3    b1
        // r2 r4 b3
        //    b3 r2 b2
        let board = Board::try_from("r2i2 r2i3 b3j1 r4j2 r3j3 g1j4 r2k1 b3k2 r1k4 b2l1 b1l3")?;
        let rdiag = Diagonal(Color::Red);
        assert_eq!(rdiag.fit(&board), btreeset!{
            btreeset!{Position::i2, Position::j3, Position::k4},
            btreeset!{Position::i3, Position::j2, Position::k1},
        });
        let bdiag = Diagonal(Color::Blue);
        assert_eq!(bdiag.fit(&board), btreeset!{
            btreeset!{Position::j1, Position::k2, Position::l3},
        });
        let ydiag = Diagonal(Color::Yellow);
        assert_eq!(ydiag.fit(&board).len(), 0);
        Ok(())
    }

    #[test]
    fn line_fit() -> Result<(), Error> {
        // b1 g1 r1
        // r2 r3 r1
        // r2 r4 r3
        //    b3 r2 b2
        let board = Board::try_from("r2i2 r2i3 b1i4 b3j1 r4j2 r3j3 g1j4 r2k1 r3k2 r1k3 r1k4 b2l1")?;
        assert_eq!(Line(Color::Red).fit(&board), btreeset!{
            btreeset!{Position::i2, Position::j2, Position::k2},
            btreeset!{Position::i3, Position::j3, Position::k3},
            btreeset!{Position::k1, Position::k2, Position::k3},
            btreeset!{Position::k2, Position::k3, Position::k4},
        });
        assert_eq!(Line(Color::Green).fit(&board).len(), 0);
        assert_eq!(Line(Color::Blue).fit(&board).len(), 0);
        assert_eq!(Line(Color::Yellow).fit(&board).len(), 0);
        Ok(())
    }

    #[test]
    fn corner_fit() -> Result<(), Error> {
        // r1 g1 r1
        // r2 r3 r1
        // r2 r4 b3
        //    b3 b2 b2
        let board = Board::try_from("r2i2 r2i3 r1i4 b3j1 r4j2 r3j3 g1j4 b2k1 b3k2 r1k3 r1k4 b2l1")?;
        assert_eq!(Corner(Color::Red).fit(&board), btreeset!{
            btreeset!{Position::i2, Position::i3, Position::j2},
            btreeset!{Position::i3, Position::i4, Position::j3},
            btreeset!{Position::i3, Position::i2, Position::j3},
            btreeset!{Position::j2, Position::i2, Position::j3},
            btreeset!{Position::j3, Position::j2, Position::i3},
            btreeset!{Position::j3, Position::k3, Position::j2},
            btreeset!{Position::k3, Position::j3, Position::k4},
        });
        assert_eq!(Corner(Color::Blue).fit(&board), btreeset!{
            btreeset!{Position::j1, Position::k1, Position::k2},
            btreeset!{Position::k1, Position::k2, Position::l1},
        });
        assert_eq!(Corner(Color::Green).fit(&board).len(), 0);
        assert_eq!(Corner(Color::Yellow).fit(&board).len(), 0);
        Ok(())
    }

    #[test]
    fn square_fit() -> Result<(), Error> {
        // r1 r1 r1 r3
        // r2 r3 r1 y4
        // r2 b4 b3
        //    b3 b2 b2
        let board = Board::try_from("r2i2 r2i3 r1i4 b3j1 b4j2 r3j3 r1j4 b2k1 b3k2 r1k3 r1k4 b2l1 y4l3 r3l4")?;
        assert_eq!(Square(Color::Red).fit(&board), btreeset!{
            btreeset!{Position::i3, Position::i4, Position::j3, Position::j4},
            btreeset!{Position::j3, Position::j4, Position::k3, Position::k4},
        });
        assert_eq!(Square(Color::Blue).fit(&board), btreeset!{
            btreeset!{Position::j1, Position::j2, Position::k1, Position::k2},
        });
        assert_eq!(Square(Color::Green).fit(&board).len(), 0);
        assert_eq!(Square(Color::Yellow).fit(&board).len(), 0);
        Ok(())
    }
}
