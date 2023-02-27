use crate::{AlgebraicNotation, Board, Location, Piece, PossibleMove, ChessLocation, Team::*};

#[test]
fn alg_in() {
    assert_eq!((6usize, 4usize).in_algebraic_notation(), String::from("e2"));
}

#[test]
fn alg_out() {
    assert_eq!(
        (1usize, 4usize),
        Location::from_algebraic_notation(String::from("e7"))
    )
}

#[test]
fn basic_pawn_moves() {
    let loc = Location::from_algebraic_notation(String::from("e2"));
    let board = Board::normal_board();
    let pos_moves = board
        .get(loc)
        .unwrap()
        .possible_moves(loc, &board);
    println!("{:?}", pos_moves);
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1))));
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1).right(1))));
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1).left(1))));
}