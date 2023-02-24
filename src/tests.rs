use crate::{AlgebraicNotation, Location};

#[test]
fn alg_in() {
    assert_eq!((1usize, 4usize).in_algebraic_notation(), String::from("e7"));
}

#[test]
fn alg_out() {
    assert_eq!(
        (1usize, 4usize),
        Location::from_algebraic_notation(String::from("e7"))
    )
}
