use crate::{Board, Step};

pub trait Piece: Copy + Sized {
    fn forwards(&self) -> (u16, u16);
    fn sideways(&self) -> (u16, u16);
    fn can_kill(&self, other: &Self) -> bool;
    fn is_king(&self) -> bool;
    fn possible_steps(&self, board: &Board<Self>) -> dyn Iterator<Item = Step>;
    fn from_fen_char() -> Option<Self>;
}
