mod board;
mod piece;
#[cfg(test)]
mod tests;

pub use board::{Board, PossibleCastles, Square};
pub use piece::{Piece, PieceKind, PossibleMove, Team};
