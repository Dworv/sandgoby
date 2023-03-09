mod status;
mod step;

use crate::{Piece, Square, BoardError::{self, *}};
pub use status::Status;
pub use step::Step;

pub struct Board<P: Piece> {
    pieces: Vec<Option<P>>,
    size: (u16, u16),
    in_bounds: Box<dyn Fn(Square) -> bool>,
    kings: Vec<Square>,
    current_team_id: u16,
    round: u32,
    halfmove_timer: u32
}

impl<P: Piece> Board<P> {
    pub fn new(
        pieces: Vec<Option<P>>,
        kings: Vec<Square>,
        size: (u16, u16),
        in_bounds: impl Fn(Square) -> bool + 'static,
        current_team_id: u16,
        round: u32,
        halfmove_timer: u32
    ) -> Result<Self, BoardError> {
        if kings.len() as u16 != P::NUM_TEAMS {
            return Err(NotEnoughKings)
        }
        if round == 0 {
            return Err(RoundIsZero)
        }
        Ok(
            Board {
                pieces, 
                size, 
                in_bounds: Box::new(in_bounds),
                kings,
                current_team_id,
                round,
                halfmove_timer
            }
        )
    }
}
