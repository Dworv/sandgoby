use crate::{Board, Square, PieceKind::*};
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
                for king_move in [(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)] {
                    let king_dest = square.forwards(self.team, king_move.0).sideways(king_move.1);
                    if king_dest.in_bounds() {
                        moves.push(PossibleMove::new(square, king_dest));
                    }
                }
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

        moves.into_iter().filter(|mve| !mve.is_illegal(board)).collect()
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

    pub fn is_illegal(&self, board: &Board) -> bool {
        let piece = board.get(self.start).unwrap();
        let team = piece.team;
        
        let king_loc = if piece.get_kind() == King { self.end } else {board.get_king_loc(team) };
        let ignoring = if piece.get_kind() == King { self.start } else { Square(-1, -1) };

        let knight_moves = [
            (2, 1),
            (2, -1),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (1, -2),
            (-1, -2)
        ];
        for mve in knight_moves {
            let knight_square = king_loc.forwards(team, mve.0).sideways(mve.1);
            if knight_square.in_bounds() {
                if let Some(other_piece) = board.get_ignoring(knight_square, ignoring) {
                    if other_piece.get_team() != team && other_piece.get_kind() == Knight {
                        return true;
                    }
                }
            }
        }
        for horz_dir in [(1, 0), (-1, 0), (0, -1), (0, 1)] {
            let mut dist = 1;
            loop {
                let current = king_loc.forwards(team, horz_dir.0 * dist).sideways(horz_dir.1 * dist);
                if !current.in_bounds() {
                    break;
                }
                if let Some(other_piece) = board.get_ignoring(current, ignoring) {
                    if team != other_piece.get_team() && (other_piece.get_kind() == Rook || other_piece.get_kind() == Queen ) {
                        return true;
                    }
                    break;
                }
                dist += 1;
            }
        }
        for vert_dir in [(1, 0), (-1, 0), (0, -1), (0, 1)] {
            let mut dist = 1;
            loop {
                let current = king_loc.forwards(team, vert_dir.0 * dist).sideways(vert_dir.1 * dist);
                if !current.in_bounds() {
                    break;
                }
                if let Some(other_piece) = board.get_ignoring(current, ignoring) {
                    if team != other_piece.get_team() && (other_piece.get_kind() == Bishop || other_piece.get_kind() == Queen ) {
                        return true;
                    }
                    break;
                }
                dist += 1;
            }
        }
        for pawn_side in [1, -1] {
            let pawn_square = king_loc.forwards(team, 1).sideways(pawn_side);
            if pawn_square.in_bounds() {
                if let Some(other_piece) = board.get_ignoring(pawn_square, ignoring) {
                    if team != other_piece.get_team() && other_piece.get_kind() == Pawn {
                        return true;
                    }
                }
            }
        }
        for king_r in -1..=1 {
            for king_c in -1..=1 {
                let king_square = king_loc.forwards(team, king_r).sideways(king_c);
                if king_square.in_bounds() {
                    if let Some(other_piece) = board.get_ignoring(king_square, ignoring) {
                        if other_piece.get_team() != team && other_piece.get_kind() == King {
                            return true;
                        }
                    }
                }
            }
        }

        false
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
