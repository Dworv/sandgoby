use std::fmt::Debug;

use crate::Piece;

#[derive(Clone, Copy, PartialEq)]
pub struct Square(pub i8, pub i8);

impl Square {
    pub fn in_alg(&self) -> String {
        format!("{}{}", ('a' as i8 + self.1) as u8 as char, 8 - self.0)
    }

    pub fn from_alg(an: &str) -> Self {
        Self(
            '8' as i8 - an.chars().nth(1).unwrap() as i8,
            an.chars().next().unwrap() as i8 - 'a' as i8,
        )
    }

    pub fn in_bounds(&self) -> bool {
        self.0 >= 0 && self.0 < 8 && self.1 < 8 && self.1 >= 0
    }

    pub fn forwards(&self, piece: &impl Piece, dist: i8) -> Self {
        let direction = piece.forwards();
        Square(self.0 + dist * direction.0, self.1 + dist * direction.1)
    }

    pub fn sideways(&self, piece: impl Piece, dist: i8) -> Self {
        let direction = piece.sideways();
        Square(self.0 + dist * direction.0, self.1 + dist * direction.1)
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.in_alg())
    }
}
