mod imp;
mod square;
pub use square::Square;

use crate::{
    Piece,
    Team, PossibleMove,
};

pub struct Board {
    pieces: [[Option<Piece>; 8]; 8],
    kings: (Square, Square),
    current_player: Team,
    castles: PossibleCastles,
    enpassent: Option<Square>,
    halfmove_clock: u8,
    num_moves: u32,
}

pub struct PossibleCastles {
    pub wq: bool,
    pub wk: bool,
    pub bq: bool,
    pub bk: bool,
}
