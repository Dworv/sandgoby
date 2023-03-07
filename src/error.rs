use std::fmt;

pub struct InvalidAlg;

impl fmt::Display for InvalidAlg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid algebraic notation")
    }
}