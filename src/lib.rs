#[macro_use] extern crate maplit;

mod board;
pub use crate::board::{Board, Position};

mod card;

mod error;
pub use crate::error::Error;

mod pattern;

mod pieces;
pub use crate::pieces::{Color, Tier, Stack};
