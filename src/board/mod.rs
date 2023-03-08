mod error;
mod status;
mod step;

use crate::{Piece, Square};
pub use error::BoardError::{self, *};
pub use status::Status;
pub use step::Step;

pub struct Board<P: Piece> {
    pieces: Vec<Option<P>>,
    size: (u16, u16),
    in_bounds: Box<dyn Fn(Square) -> bool>
}

impl<P: Piece> Board<P> {
    pub fn assemble(
        fen: &str,
        size: (u16, u16),
        in_bounds: impl Fn(Square) -> bool + 'static
    ) -> Result<Self, BoardError> {
        let mut board = Board {
            pieces: vec![],
            size,
            in_bounds: Box::new(in_bounds)
        };

        let mut fen_pieces = fen.split(' ');
        let mut board_rows = fen_pieces.next().ok_or(InvalidFen)?.split('/');

        

        let raw_turn = fen_pieces.next().ok_or(InvalidFen)?;
        let raw_castle_rights = fen_pieces.next().ok_or(InvalidFen)?;
        let raw_enpassent_square = fen_pieces.next().ok_or(InvalidFen)?;
        let num_halfmoves = fen_pieces.next().ok_or(InvalidFen)?;
        let num_rounds = fen_pieces.next().ok_or(InvalidFen)?;

        Ok(board)
    }
}
