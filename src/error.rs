use std::fmt;

#[derive(Debug)]
pub struct InvalidAlg;

impl fmt::Display for InvalidAlg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid algebraic notation")
    }
}

use std::fmt::Display;

pub enum BoardError {
    NotEnoughKings,
    IllegalPosition,
    InvalidFen,
    UnknownError,
    RoundIsZero
}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::IllegalPosition => "Illegal position",
            Self::InvalidFen => "Invalid fen",
            Self::UnknownError => "Unknown error (please report)",
            Self::NotEnoughKings => "Not enough kings on the board",
            Self::RoundIsZero => "Round is 0 (needs to be 1 or higher)"
        })
    }
}