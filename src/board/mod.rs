mod status;
mod step;

use crate::{Piece, Square, BoardError};
pub use status::Status;
pub use step::Step;

pub struct Board<P: Piece> {
    pieces: Vec<Option<P>>,
    size: (u16, u16),
    in_bounds: Box<dyn Fn(Square) -> bool>,
    kings: Vec<Square>,
    turn_team_id: u16,
    round: u32,
    halfmove_timer: u32
}

impl<P: Piece> Board<P> {
    pub fn assemble(
        fen: &str,
        size: (u16, u16),
        in_bounds: impl Fn(Square) -> bool + 'static
    ) -> Result<Self, BoardError> {
        

        Ok(
            Board {
                pieces, 
                size, 
                in_bounds: Box::new(in_bounds),
                kings,
                turn_team_id,
                round,
                halfmove_timer
            }
        )
    }
}
