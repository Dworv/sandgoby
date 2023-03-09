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
    IllegalPosition,
    InvalidFen,
    UnknownError
}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::IllegalPosition => "Illegal position",
            Self::InvalidFen => "Invalid fen",
            Self::UnknownError => "Unknown error (please report)"
        })
    }
}