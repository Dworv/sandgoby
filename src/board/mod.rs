mod error;
mod status;
mod step;

use crate::{Piece, Square};
pub use error::BoardError::{self, *};
pub use status::Status;
pub use step::Step;

pub struct Board<P: Piece> {
    pieces: Vec<Option<P>>,
    size: (u16, u16),
    in_bounds: Box<dyn Fn(Square) -> bool>,
    kings: Vec<Square>,
    turn_team_id: u16,
    round: u32,
    halfmove_timer: u32
}

impl<P: Piece> Board<P> {
    pub fn assemble(
        fen: &str,
        size: (u16, u16),
        in_bounds: impl Fn(Square) -> bool + 'static
    ) -> Result<Self, BoardError> {
        let mut fen_pieces = fen.split(' ');

        let board_rows = fen_pieces.next().ok_or(InvalidFen)?.split('/');
        let mut pieces = Vec::with_capacity((size.0 * size.1) as usize);
        let mut kings_maybes = vec![None; P::NUM_TEAMS as usize];

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
                let piece = P::from_fen_char(*ch as char).ok_or(InvalidFen)?;
                let index = size.1 as usize * r + c;
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
            if c as u16 != size.1 - 1 {
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
        let turn_team_id = if let Some(team_id) = P::team_id_from_fen(raw_turn.chars().next().ok_or(InvalidFen)?) {
            team_id
        } else {
            return Err(InvalidFen);
        };

        let raw_castle_rights = fen_pieces.next().ok_or(InvalidFen)?;
        let raw_enpassent_square = fen_pieces.next().ok_or(InvalidFen)?;
        let halfmove_timer = fen_pieces.next().ok_or(InvalidFen)?.parse::<u32>().map_err(|_| InvalidFen)?;
        let round = fen_pieces.next().ok_or(InvalidFen)?.parse::<u32>().map_err(|_| InvalidFen)?;

        Ok(
            Board {
                pieces, 
                size, 
                in_bounds: Box::new(in_bounds),
                kings,
                turn_team_id,
                round,
                halfmove_timer
            }
        )
    }
}
