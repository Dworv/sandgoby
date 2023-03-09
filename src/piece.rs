use crate::{Board, Step};

pub trait Piece: Copy + Sized {
    const NUM_TEAMS: u16;
    type StepIter: Iterator<Item = Step>;
    fn forwards(&self) -> (u16, u16);
    fn sideways(&self) -> (u16, u16);
    fn can_kill(&self, other: &Self) -> bool;
    fn is_king(&self) -> bool;
    fn team_id(&self) -> u16;
    fn possible_steps(&self, board: &Board<Self>) -> Self::StepIter;
}
