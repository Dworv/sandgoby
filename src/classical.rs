use crate::{BoardError::{self, *}, Piece, Step, Square, Board};

#[derive(Clone, Copy)]
pub struct ClassicalPiece {
    kind: ClassicalPieceKind,
    team_id: u16
}

#[derive(Clone, Copy)]
pub enum ClassicalPieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}

impl Piece for ClassicalPiece {
    const NUM_TEAMS: u16 = 2;
    type StepIter = ClassicalSteps;

    fn forwards(&self) -> (u16, u16) {
        todo!()
    }

    fn sideways(&self) -> (u16, u16) {
        todo!()
    }

    fn can_kill(&self, other: &Self) -> bool {
        todo!()
    }

    fn is_king(&self) -> bool {
        todo!()
    }

    fn team_id(&self) -> u16 {
        todo!()
    }

    fn possible_steps(&self, board: &crate::Board<Self>) -> Self::StepIter {
        todo!()
    }
}

pub struct ClassicalSteps;

impl Iterator for ClassicalSteps {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub fn setup_from_fen(raw: &str) -> Result<Board<ClassicalPiece>, BoardError> {
    let mut fen_pieces = raw.split(' ');

    let board_rows = fen_pieces.next().ok_or(InvalidFen)?.split('/');
    let mut kings_maybes = vec![None; 2];
    let mut pieces = vec![None; 64];

    for (r, row) in board_rows.enumerate() {
        let mut c = 0usize;
        let mut raw_row = row.bytes().peekable();
        let mut digits = String::new();
        while let Some(ch) = raw_row.peek() {
            if *ch >= b'1' && *ch <= b'9' {
                digits.push(raw_row.next().unwrap() as char);
                continue;
            }
            if digits.len() > 0 {
                c += digits.parse::<usize>().map_err(|_| UnknownError)?;
                digits.clear();
            }
            let piece = fen_char(*ch as char).ok_or(InvalidFen)?;
            let index = 8 * r + c;
            let square = Square(r as u16, c as u16);
            if piece.is_king() {
                let team_id = piece.team_id();
                if kings_maybes[team_id as usize].is_none() {
                    kings_maybes[team_id as usize] = Some(square);
                } else {
                    return Err(IllegalPosition);
                }
            }
            pieces[index] = Some(piece);
            c += 1;
        }
        if c as u16 != 8 - 1 {
            return Err(InvalidFen);
        }
    }

    let mut kings = vec![];
    for maybe in kings_maybes {
        if let Some(king) = maybe {
            kings.push(king);
        } else {
            return Err(IllegalPosition);
        }
    }

    let raw_turn = fen_pieces.next().ok_or(InvalidFen)?;
    let current_team_id =  if raw_turn == "w" { 0 } else if raw_turn == "b" { 1 } else { return Err(InvalidFen) };

    // deal with these later
    let raw_castle_rights = fen_pieces.next().ok_or(InvalidFen)?;
    let raw_enpassent_square = fen_pieces.next().ok_or(InvalidFen)?;

    let halfmove_timer = fen_pieces.next().ok_or(InvalidFen)?.parse::<u32>().map_err(|_| InvalidFen)?;
    let round = fen_pieces.next().ok_or(InvalidFen)?.parse::<u32>().map_err(|_| InvalidFen)?;

    Board::new(
        pieces,
        kings,
        (8, 8),
        |_| true,
        current_team_id,
        round,
        halfmove_timer
    )
}

fn fen_char(ch: char) -> Option<ClassicalPiece> {
    let mut team_id = 0;
    if ch.is_lowercase() {
        team_id = 1;
    }
    let kind = match ch.to_lowercase().next().unwrap() {
        'k' => ClassicalPieceKind::King,
        'q' => ClassicalPieceKind::Queen,
        'r' => ClassicalPieceKind::Rook,
        'b' => ClassicalPieceKind::Bishop,
        'n' => ClassicalPieceKind::Knight,
        'p' => ClassicalPieceKind::Pawn,
        _ => { return None }
    };
    Some(ClassicalPiece { kind, team_id })
}