use crate::{Board, Step, BoardError};

pub trait Piece: Copy + Sized {
    const NUM_TEAMS: u16;
    fn forwards(&self) -> (u16, u16);
    fn sideways(&self) -> (u16, u16);
    fn can_kill(&self, other: &Self) -> bool;
    fn is_king(&self) -> bool;
    fn team_id(&self) -> u16;
    fn possible_steps(&self, board: &Board<Self>) -> dyn Iterator<Item = Step>;
    fn str_to_pieces(raw: &str, size: (u16, u16), in_bounds: impl Fn((u16, u16)) -> bool + 'static) ->  Result<Vec<Option<Self>>, BoardError>;
}
