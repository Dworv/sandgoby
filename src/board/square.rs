use std::fmt::Debug;
use crate::Team::{self, *};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(pub i8, pub i8);

impl Square {
    pub fn in_alg(&self) -> String {
        format!("{}{}", ('a' as i8 + self.1) as u8 as char, 8 - self.0)
    }

    pub fn from_alg(an: &str) -> Self {
        Self (
            '8' as i8 - an.chars().nth(1).unwrap() as i8,
            an.chars().next().unwrap() as i8 - 'a' as i8,
        )
    }

    pub fn in_bounds(&self) -> bool {
        self.0 >= 0 && self.0 < 8 && self.1 < 8 && self.1 >= 0
    }

    pub fn forwards(&self, team: Team, dist: i8) -> Self {
        match team {
            White => Self(self.0 - dist, self.1),
            Black => Self(self.0 + dist, self.1),
        }
    }

    pub fn sideways(&self, dist: i8) -> Self {
        Self(self.0, self.1 + dist)
    }
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.in_alg())
    }
}
