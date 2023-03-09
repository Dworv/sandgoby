mod board;
mod classical;
mod error;
mod piece;
mod square;

#[cfg(test)]
mod tests;

pub use board::{Board, Status, Step};
pub use error::{InvalidAlg, BoardError::{self, *}};
pub use piece::Piece;
pub use square::Square;
