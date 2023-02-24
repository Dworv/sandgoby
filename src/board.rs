use std::fmt::{self, Debug, Formatter};

use crate::{
    Piece,
    PieceKind::*,
    Team::{self, *},
};

pub struct Board {
    pieces: [[Option<Piece>; 8]; 8],
    current_player: Team,
    castles: Castles,
    enpassent: Option<(usize, usize)>,
    halfmove_clock: u8,
    num_moves: u32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            pieces: Default::default(),
            current_player: White,
            castles: Castles {
                wq: false,
                wk: false,
                bq: false,
                bk: false,
            },
            enpassent: None,
            halfmove_clock: 0,
            num_moves: 1
        }
    }

    pub fn normal_board() -> Self {
        Board::from_fen("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50")
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut contents = fen.split(" ");
        let mut board = Board::new();
        for (r, row) in contents.next().unwrap().split("/").enumerate() {
            let mut c = 0usize;
            for ch in row.chars() {
                if (ch as u8 > '0' as u8) && ('9' as u8 > ch as u8) {
                    c += ch as usize - '0' as usize;
                } else {
                    board.insert(
                        (r, c),
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
            if castling.contains("K") {
                board.castles.wk = true
            }
            if castling.contains("Q") {
                board.castles.wq = true
            }
            if castling.contains("k") {
                board.castles.bk = true
            }
            if castling.contains("q") {
                board.castles.bq = true
            }
        }
        let enpassent = contents.next().unwrap();
        if enpassent != "-" {
            board.enpassent = Some(Location::from_algebraic_notation(String::from(enpassent)))
        }
        if let Ok(num) = contents.next().unwrap().parse() {
            board.halfmove_clock = num
        }
        if let Ok(num) = contents.next().unwrap().parse() {
            board.num_moves = num
        }
        board
    }

    pub fn insert(&mut self, location: (usize, usize), piece: Piece) {
        self.pieces[location.0][location.1] = Some(piece);
    }

    pub fn remove(&mut self, location: (usize, usize)) {
        self.pieces[location.0][location.1] = None;
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

pub type Location = (usize, usize);

pub trait AlgebraicNotation {
    fn from_algebraic_notation(an: String) -> Self;
    fn in_algebraic_notation(&self) -> String;
}

impl AlgebraicNotation for Location {
    fn in_algebraic_notation(&self) -> String {
        format!("{}{}", ('a' as usize + self.1) as u8 as char, 8 - self.0)
    }

    fn from_algebraic_notation(an: String) -> Self {
        (
            '8' as usize - an.chars().nth(1).unwrap() as usize,
            an.chars().nth(0).unwrap() as usize - 'a' as usize,
        )
    }
}
