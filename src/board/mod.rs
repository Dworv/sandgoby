mod square;
pub use square::Square;

use std::fmt::{self, Debug, Formatter};

use crate::{
    Piece,
    PieceKind::{*, self},
    Team::{self, *},
};

pub struct Board {
    pieces: [[Option<Piece>; 8]; 8],
    kings: (Square, Square),
    current_player: Team,
    castles: Castles,
    enpassent: Option<Square>,
    halfmove_clock: u8,
    num_moves: u32,
}

impl Board {
    pub fn new() -> Self {
        let mut pieces: [[Option<Piece>; 8]; 8] = Default::default();
        pieces[0][4] = Some(Piece::new(Black, King));
        pieces[7][4] = Some(Piece::new(White, King));
        Board {
            pieces,
            kings: (Square(7, 4), Square(0, 4)),
            current_player: White,
            castles: Castles {
                wq: false,
                wk: false,
                bq: false,
                bk: false,
            },
            enpassent: None,
            halfmove_clock: 0,
            num_moves: 1,
        }
    }

    pub fn normal_board() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut contents = fen.split(' ');
        let mut board = Board::new();
        let mut kings_added = (false, false);
        for (r, row) in contents.next().unwrap().split('/').enumerate() {
            let mut c = 0usize;
            for ch in row.chars() {
                if (ch as u8 > b'0') && (b'9' > ch as u8) {
                    c += ch as usize - '0' as usize;
                } else {
                    if ch == 'K' {
                        if !kings_added.0 {
                            kings_added.0 = true;
                            board.kings.0 = Square(r as i8, c as i8)
                        } else {
                            panic!("TWO KINGS")
                        }
                    } else if ch == 'k' {
                        if !kings_added.1 {
                            kings_added.1 = true;
                            board.kings.1 = Square(r as i8, c as i8)
                        } else {
                            panic!("TWO KINGS")
                        }
                    }
                    board.insert(
                        Square(r as i8, c as i8),
                        match ch {
                            'K' => Piece::new(White, King),
                            'Q' => Piece::new(White, Queen),
                            'R' => Piece::new(White, Rook),
                            'B' => Piece::new(White, Bishop),
                            'N' => Piece::new(White, Knight),
                            'P' => Piece::new(White, Pawn),
                            'k' => Piece::new(Black, King),
                            'q' => Piece::new(Black, Queen),
                            'r' => Piece::new(Black, Rook),
                            'b' => Piece::new(Black, Bishop),
                            'n' => Piece::new(Black, Knight),
                            'p' => Piece::new(Black, Pawn),
                            _ => panic!(),
                        },
                    );
                    c += 1;
                }
            }
        }
        board.current_player = match contents.next().unwrap() {
            "w" => White,
            "b" => Black,
            _ => panic!(),
        };
        let castling = contents.next().unwrap();
        if castling != "-" {
            if castling.contains('K') {
                board.castles.wk = true
            }
            if castling.contains('Q') {
                board.castles.wq = true
            }
            if castling.contains('k') {
                board.castles.bk = true
            }
            if castling.contains('q') {
                board.castles.bq = true
            }
        }
        let enpassent = contents.next().unwrap();
        if enpassent != "-" {
            board.enpassent = Some(Square::from_alg(enpassent))
        }
        if let Ok(num) = contents.next().unwrap().parse() {
            board.halfmove_clock = num
        }
        if let Ok(num) = contents.next().unwrap().parse() {
            board.num_moves = num
        }
        board
    }

    pub fn get(&self, square: Square) -> Option<Piece> {
        self.pieces[square.0 as usize][square.1 as usize]
    }

    pub fn insert(&mut self, square: Square, piece: Piece) {
        self.pieces[square.0 as usize][square.1 as usize] = Some(piece);
    }

    pub fn remove(&mut self, square: Square) {
        self.pieces[square.0 as usize][square.1 as usize] = None;
    }

    pub fn enpassent(&self) -> Option<Square> {
        self.enpassent
    }

    pub fn get_king_loc(&self, team: Team) -> Square {
        if team == White { self.kings.0 } else {self.kings.1}
    }

    pub fn set_king_loc(&mut self, team: Team, square: Square) {
        if team == White { 
            self.kings.0 = square;
        } else { 
            self.kings.1 = square;
        }
    }

    pub fn threatened(&self, square: Square, ignoring: Option<Square>) -> bool {
        if let Some(piece) = self.get(square) {
            let team = piece.get_team();
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
                let new_square = square.forwards(team, mve.0).sideways(mve.1);
                if new_square.in_bounds() {
                    if let Some(other_piece) = self.get(new_square) {
                        if other_piece.get_team() != team && other_piece.get_kind() == Knight {
                            return true;
                        }
                    }
                }
            }
            for offset in [(1, 0), (-1, 0), (0, -1), (0, 1)] {
                let mut dist = 1;
                loop {
                    let current = square.forwards(team, offset.0 * dist).sideways(offset.1 * dist);
                    if !current.in_bounds() {
                        break;
                    }
                    if let Some(other_piece) = self.get(current) {
                        if team != other_piece.get_team() && (other_piece.get_kind() == Rook || other_piece.get_kind() == Queen ) {
                            return true;
                        }
                        break;
                    }
                    dist += 1;
                }
            }
            for offset in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                let mut dist = 1;
                loop {
                    let current = square.forwards(team, offset.0 * dist).sideways(offset.1 * dist);
                    if !current.in_bounds() {
                        break;
                    }
                    if let Some(other_piece) = self.get(current) {
                        if team != other_piece.get_team() && (other_piece.get_kind() == Bishop || other_piece.get_kind() == Queen ) {
                            return true;
                        }
                        break;
                    }
                    dist += 1;
                }
            }
            for pawn_side in [1, -1] {
                if let Some(other_piece) = self.get(square.forwards(team, 1).sideways(pawn_side)) {
                    if team != other_piece.get_team() && other_piece.get_kind() == Pawn {
                        return true;
                    }
                }
            }
            for king_r in -1..=1 {
                for king_c in -1..=1 {
                    if let Some(other_piece) = self.get(square.forwards(team, king_r).sideways(king_c)) {
                        if other_piece.get_team() != team && other_piece.get_kind() == King {
                            return true;
                        }
                    }
                }
            }
        } else {
            panic!("nothing here :(");
        }
        false
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.pieces {
            for piece in row {
                match piece {
                    Some(p) => p.fmt(f)?,
                    None => f.write_str("[]")?,
                }
                f.write_str(" ")?
            }
            f.write_str("\n")?
        }
        fmt::Result::Ok(())
    }
}

pub struct Castles {
    pub wq: bool,
    pub wk: bool,
    pub bq: bool,
    pub bk: bool,
}
