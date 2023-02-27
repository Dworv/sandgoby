use crate::{Board, ChessLocation, Location};
use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy)]
pub struct Piece {
    team: Team,
    kind: PieceKind,
}

impl Piece {
    pub fn new(team: Team, kind: PieceKind) -> Self {
        Self { team, kind }
    }
    pub fn possible_moves(&self, location: Location, board: &Board) -> Vec<PossibleMove> {
        let mut moves = vec![];
        match self.kind {
            PieceKind::King => todo!(),
            PieceKind::Queen => todo!(),
            PieceKind::Rook => todo!(),
            PieceKind::Bishop => todo!(),
            PieceKind::Knight => todo!(),
            PieceKind::Pawn => {
                if board.get(location.forwards(self.team, 1)).is_none() {
                    moves.push(PossibleMove::new(location, location.forwards(self.team, 1)))
                }
                let takes = [
                    location.forwards(self.team, 1).left(1),
                    location.forwards(self.team, 1).right(1),
                ];
                println!("{takes:?}");
                for take in takes {
                    if take.in_bounds() {
                        if let Some(target) = board.get(take) {
                            if target.team != self.team {
                                moves.push(PossibleMove::new(location, take));
                            }
                        } else if let Some(enp) = board.enpassent() {
                            if enp == take {
                                moves.push(PossibleMove::new(location, take));
                            }
                        }
                    }
                }
                if !location.forwards(self.team, 2).in_bounds() {
                    for pmove in &mut moves {
                        pmove.set_promotion(true)
                    }
                }
            }
        }
        moves
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self.team {
                Team::White => "W",
                Team::Black => "B",
            },
            match self.kind {
                PieceKind::King => "K",
                PieceKind::Queen => "Q",
                PieceKind::Rook => "R",
                PieceKind::Bishop => "B",
                PieceKind::Knight => "N",
                PieceKind::Pawn => "P",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Team {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub enum CastleKind {
    Kingside,
    Queenside,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PossibleMove {
    start: Location,
    end: Location,
    rook_start: Option<Location>,
    rook_end: Option<Location>,
    promotion: bool,
}

impl PossibleMove {
    pub fn new(start: Location, end: Location) -> Self {
        Self {
            start,
            end,
            rook_start: None,
            rook_end: None,
            promotion: false,
        }
    }

    pub fn set_rook(&mut self, start: Location, end: Location) {
        self.rook_start = Some(start);
        self.rook_end = Some(end);
    }

    pub fn set_promotion(&mut self, promotion: bool) {
        self.promotion = promotion;
    }
}

pub struct Move {
    start: Location,
    end: Location,
    check: bool,
    capture: bool,
    checkmate: bool,
    kind: MoveKind,
}

pub enum MoveKind {
    Regular,
    Enpassent,
    Castle {
        rook_start: Location,
        rook_end: Location,
    },
    Promotion {
        new_piece: PieceKind,
    },
}
