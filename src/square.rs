use std::fmt::Debug;

use crate::{Piece, InvalidAlg};

#[derive(Clone, Copy, PartialEq)]
pub struct Square(pub u16, pub u16);

impl Square {
    pub fn in_alg(&self) -> String {
        format!("{}{}", ('a' as u16 + self.1) as u8 as char, 8 - self.0)
    }

    pub fn try_from_alg(an: &str, size: (u16, u16)) -> Result<Self, InvalidAlg> {
        let mut chars = an.bytes().peekable();
        let mut digits = String::new();
        let mut letters = vec![];
        if an.len() < 2 {
            return Err(InvalidAlg);
        }

        while let Some(c) = chars.peek() {
            if *c >= b'a' && *c <= b'z' {
                letters.push(chars.next().unwrap() - b'a')
            } else {
                break;
            }
        }

        while let Some(c) = chars.peek() {
            if *c <= b'9' && *c >= b'0' {
                digits.push(chars.next().unwrap() as char);
            } else {
                chars.next();
            }
        }

        if digits.len() == 0 || letters.len() == 0 {
            return Err(InvalidAlg);
        }

        let row = size.0 - digits.parse::<u16>().map_err(|_| InvalidAlg)?;
        // TODO: Improve and allow more than 26 columns
        dbg!(&letters);
        let col = letters[0] as u16;
        Ok(Self(row, col))
    }

    pub fn in_bounds(self, size: (u16, u16), boundary: &dyn Fn(Self) -> bool) -> bool {
        self.0 < size.0 && self.1 < size.1 && boundary(self)
    }

    pub fn forwards(self, piece: &impl Piece, dist: u16) -> Option<Self> {
        let direction = piece.forwards();
        let mut square = self;
        match direction.0 {
            DirScale::Plus => {
                square = Square(square.0.checked_add(dist)?, square.1);
            },
            DirScale::Minus => {
                square = Square(square.0.checked_sub(dist)?, square.1);
            },
            _ => {},
        }
        match direction.1 {
            DirScale::Plus => {
                square = Square(square.0, square.1.checked_add(dist)?);
            },
            DirScale::Minus => {
                square = Square(square.0, square.1.checked_sub(dist)?);
            },
            _ => {},
        }
        Some(square)
    }

    pub fn sideways(self, piece: &impl Piece, dist: u16) -> Option<Self> {
        let direction = piece.sideways();
        let mut square = self;
        match direction.0 {
            DirScale::Plus => {
                square = Square(square.0.checked_add(dist)?, square.1);
            },
            DirScale::Minus => {
                square = Square(square.0.checked_sub(dist)?, square.1);
            },
            _ => {},
        }
        match direction.1 {
            DirScale::Plus => {
                square = Square(square.0, square.1.checked_add(dist)?);
            },
            DirScale::Minus => {
                square = Square(square.0, square.1.checked_sub(dist)?);
            },
            _ => {},
        }
        Some(square)
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.in_alg())
    }
}

#[derive(Clone, Copy)]
pub struct Direction(pub DirScale, pub DirScale);

#[derive(Clone, Copy)]
pub enum DirScale {
    Plus,
    Minus,
    Neutral
}