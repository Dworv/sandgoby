use std::fmt::Display;

pub enum BoardError {
    IllegalPosition,
    InvalidFen,
}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::IllegalPosition => "Illegal position",
            Self::InvalidFen => "Invalid fen"
        })
    }
}