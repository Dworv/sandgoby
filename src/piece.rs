use crate::{Board, Square};
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

    pub fn possible_moves(&self, square: Square, board: &Board) -> Vec<PossibleMove> {
        let mut moves = vec![];
        match self.kind {
            PieceKind::King => {

            },
            PieceKind::Queen => {
                for offset in [(1, 1), (-1, 1), (1, -1), (-1, -1), (1, 0), (-1, 0), (0, -1), (0, 1)] {
                    let mut dist = 1;
                    loop {
                        let current = square.forwards(self.team, offset.0 * dist).sideways(offset.1 * dist);
                        if !current.in_bounds() {
                            break;
                        }
                        match board.get(current) {
                            Some(piece) => {
                                if piece.team != self.team {
                                    moves.push(PossibleMove::new(square, current));
                                }
                                break;
                            },
                            None => {
                                moves.push(PossibleMove::new(square, current));
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
                        let current = square.forwards(self.team, offset.0 * dist).sideways(offset.1 * dist);
                        if !current.in_bounds() {
                            break;
                        }
                        match board.get(current) {
                            Some(piece) => {
                                if piece.team != self.team {
                                    moves.push(PossibleMove::new(square, current));
                                }
                                break;
                            },
                            None => {
                                moves.push(PossibleMove::new(square, current));
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
                        let current = square.forwards(self.team, offset.0 * dist).sideways(offset.1 * dist);
                        if !current.in_bounds() {
                            break;
                        }
                        match board.get(current) {
                            Some(piece) => {
                                if piece.team != self.team {
                                    moves.push(PossibleMove::new(square, current));
                                }
                                break;
                            },
                            None => {
                                moves.push(PossibleMove::new(square, current));
                            },
                        }
                        dist += 1;
                    }
                }
            },
            PieceKind::Knight => {
                moves.append(
                    &mut [
                        square.forwards(self.team, 2).sideways(-1),
                        square.forwards(self.team, 2).sideways(1),
                        square.forwards(self.team, -2).sideways(-1),
                        square.forwards(self.team, -2).sideways(1),
                        square.forwards(self.team, 1).sideways(2),
                        square.forwards(self.team, -1).sideways(2),
                        square.forwards(self.team, 1).sideways(-2),
                        square.forwards(self.team, -1).sideways(-2)
                    ].into_iter()
                        .filter(|loc| loc.in_bounds())
                        .map(|end| PossibleMove::new(square, end))
                        .collect()
                );
            }
            PieceKind::Pawn => {
                if board.get(square.forwards(self.team, 1)).is_none() {
                    moves.push(PossibleMove::new(square, square.forwards(self.team, 1)))
                }
                let takes = [
                    square.forwards(self.team, 1).sideways(-1),
                    square.forwards(self.team, 1).sideways(1),
                ];
                println!("{takes:?}");
                for take in takes {
                    if take.in_bounds() {
                        if let Some(target) = board.get(take) {
                            if target.team != self.team {
                                moves.push(PossibleMove::new(square, take));
                            }
                        } else if let Some(enp) = board.enpassent() {
                            if enp == take {
                                moves.push(PossibleMove::new(square, take));
                            }
                        }
                    }
                }
                if !square.forwards(self.team, 2).in_bounds() {
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
    start: Square,
    end: Square,
    rook_start: Option<Square>,
    rook_end: Option<Square>,
    promotion: bool,
}

impl PossibleMove {
    pub fn new(start: Square, end: Square) -> Self {
        Self {
            start,
            end,
            rook_start: None,
            rook_end: None,
            promotion: false,
        }
    }

    pub fn set_rook(&mut self, start: Square, end: Square) {
        self.rook_start = Some(start);
        self.rook_end = Some(end);
    }

    pub fn set_promotion(&mut self, promotion: bool) {
        self.promotion = promotion;
    }
}

pub struct Move {
    start: Square,
    end: Square,
    check: bool,
    capture: bool,
    checkmate: bool,
    kind: MoveKind,
}

pub enum MoveKind {
    Regular,
    Enpassent,
    Castle {
        rook_start: Square,
        rook_end: Square,
    },
    Promotion {
        new_piece: PieceKind,
    },
}
