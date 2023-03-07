use crate::{Board, Step};

pub trait Piece: Sized {
    fn forwards(&self) -> (i8, i8);
    fn sideways(&self) -> (i8, i8);
    fn can_kill(&self, other: &Self) -> bool;
    fn possible_steps(&self, board: &Board<Self>) -> dyn Iterator<Item = Step>;
}
