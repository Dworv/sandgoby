use crate::{Board, Step, square::Direction, Square};

pub trait Piece: Copy + Sized {
    const NUM_TEAMS: u16;
    const BOARD_SIZE: (u16, u16);
    type StepIter: Iterator<Item = Step>;
    fn in_bounds(square: Square) -> bool;
    fn forwards(&self) -> Direction;
    fn sideways(&self) -> Direction;
    fn can_kill(&self, other: &Self) -> bool;
    fn is_king(&self) -> bool;
    fn team_id(&self) -> u16;
    fn possible_steps(&self, board: &Board<Self>) -> Self::StepIter;
}
