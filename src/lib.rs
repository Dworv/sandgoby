mod board;
mod piece;
#[cfg(test)]
mod tests;

pub use board::{AlgebraicNotation, Board, ChessLocation, Location};
pub use piece::{Piece, PieceKind, PossibleMove, Team};
