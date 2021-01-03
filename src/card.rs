use std::convert::TryFrom;
use crate::{Board, Color, Error};
use crate::pattern::{AdjacentColors, AdjacentT2, Corner, DiagonalStacks, Diagonal, Line, Pattern, Square, Surround};

struct Card {
    pattern: Box<dyn Pattern>,
    value: u32,
    pieces: [Color; 2],
}

impl Card {
    fn score(&self, board: &Board) -> u32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_test() -> Result<(), Error> {
        Ok(())
    }
}
