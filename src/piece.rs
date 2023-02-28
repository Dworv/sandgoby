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

    pub fn get_team(&self) -> Team {
        self.team
    }

    pub fn get_kind(&self) -> PieceKind {
        self.kind
    }

    pub fn possible_moves(&self, location: Location, board: &Board) -> Vec<PossibleMove> {
        let mut moves = vec![];
        match self.kind {
            PieceKind::King => {

            },
            PieceKind::Queen => {
                for offset in [(1, 1), (-1, 1), (1, -1), (-1, -1), (1, 0), (-1, 0), (0, -1), (0, 1)] {
                    let mut dist = 1;
                    loop {
                        let current = location.forwards(self.team, offset.0 * dist).sideways(offset.1 * dist);
                        if !current.in_bounds() {
                            break;
                        }
                        match board.get(current) {
                            Some(piece) => {
                                if piece.team != self.team {
                                    moves.push(PossibleMove::new(location, current));
                                }
                                break;
                            },
                            None => {
                                moves.push(PossibleMove::new(location, current));
                            },
                        }
                        dist += 1;
                    }
                }
            },
            PieceKind::Rook => {
                for offset in [(1, 0), (-1, 0), (0, -1), (0, 1)] {
                    let mut dist = 1;
                    loop {
                        let current = location.forwards(self.team, offset.0 * dist).sideways(offset.1 * dist);
                        if !current.in_bounds() {
                            break;
                        }
                        match board.get(current) {
                            Some(piece) => {
                                if piece.team != self.team {
                                    moves.push(PossibleMove::new(location, current));
                                }
                                break;
                            },
                            None => {
                                moves.push(PossibleMove::new(location, current));
                            },
                        }
                        dist += 1;
                    }
                }
            },
            PieceKind::Bishop => {
                for offset in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                    let mut dist = 1;
                    loop {
                        let current = location.forwards(self.team, offset.0 * dist).sideways(offset.1 * dist);
                        if !current.in_bounds() {
                            break;
                        }
                        match board.get(current) {
                            Some(piece) => {
                                if piece.team != self.team {
                                    moves.push(PossibleMove::new(location, current));
                                }
                                break;
                            },
                            None => {
                                moves.push(PossibleMove::new(location, current));
                            },
                        }
                        dist += 1;
                    }
                }
            },
            PieceKind::Knight => {
                moves.append(
                    &mut [
                        location.forwards(self.team, 2).sideways(-1),
                        location.forwards(self.team, 2).sideways(1),
                        location.forwards(self.team, -2).sideways(-1),
                        location.forwards(self.team, -2).sideways(1),
                        location.forwards(self.team, 1).sideways(2),
                        location.forwards(self.team, -1).sideways(2),
                        location.forwards(self.team, 1).sideways(-2),
                        location.forwards(self.team, -1).sideways(-2)
                    ].into_iter()
                        .filter(|loc| loc.in_bounds())
                        .map(|end| PossibleMove::new(location, end))
                        .collect()
                );
            }
            PieceKind::Pawn => {
                if board.get(location.forwards(self.team, 1)).is_none() {
                    moves.push(PossibleMove::new(location, location.forwards(self.team, 1)))
                }
                let takes = [
                    location.forwards(self.team, 1).sideways(-1),
                    location.forwards(self.team, 1).sideways(1),
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

#[derive(Clone, Copy, PartialEq, Eq)]
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
