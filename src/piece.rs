use crate::{Board, Location};
use std::fmt::{Debug, Formatter};

pub struct Piece {
    team: Team,
    kind: PieceKind,
}

impl Piece {
    pub fn new(team: Team, kind: PieceKind) -> Self {
        Self { team, kind }
    }
    pub fn moves(&self, location: Location, board: &Board) -> Vec<Move> {
        match self.kind {
            PieceKind::King => {
                if
            },
            PieceKind::Queen => todo!(),
            PieceKind::Rook => todo!(),
            PieceKind::Bishop => todo!(),
            PieceKind::Knight => todo!(),
            PieceKind::Pawn => todo!(),
        }
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

pub enum Team {
    White,
    Black,
}

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
    Queenside
}

pub enum Move {
    Regular { start: Location, end: Location },
    Castle { kind: CastleKind },
    Enpassent { start: Location, end: Location }
}