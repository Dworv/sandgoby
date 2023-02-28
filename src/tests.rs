use crate::{AlgebraicNotation, Board, ChessLocation, Location, PossibleMove, Team::*};

#[test]
fn alg_in() {
    assert_eq!((6i8, 4i8).in_algebraic_notation(), String::from("e2"));
}

#[test]
fn alg_out() {
    assert_eq!(
        (1i8, 4i8),
        Location::from_algebraic_notation(String::from("e7"))
    )
}

#[test]
fn basic_pawn_moves() {
    let loc = Location::from_algebraic_notation(String::from("e2"));
    let board = Board::normal_board();
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1))));
}

#[test]
fn pawn_taking() {
    let loc = Location::from_algebraic_notation(String::from("d5"));
    let board = Board::from_fen("7k/8/4p3/2pP4/8/8/8/7K w - c6 1 1");
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1))));
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1).sideways(-1))));
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1).sideways(1))));
}

#[test]
fn pawn_promotion() {
    let loc = Location::from_algebraic_notation(String::from("a7"));
    let board = Board::from_fen("7k/P7/8/8/8/8/8/7K w - - 1 1");
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    println!("{:?}", pos_moves);
    let mut promote = PossibleMove::new(loc, loc.forwards(White, 1));
    promote.set_promotion(true);
    assert!(pos_moves.contains(&promote));
}

#[test]
fn knight_moves() {
    let loc = Location::from_algebraic_notation(String::from("c6"));
    let board = Board::from_fen("7k/8/2N5/8/8/8/8/7K w - - 1 1");
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    println!("{pos_moves:?}");
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 2).sideways(1))));
}

#[test]
fn bishop_moves() {
    let loc = Location::from_algebraic_notation(String::from("c6"));
    let board = Board::from_fen("7k/1p1P4/2B5/8/8/8/8/7K w - - 1 1");
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    println!("{pos_moves:?}");
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, -2).sideways(2))));
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1).sideways(-1))));
    assert!(!pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 2).sideways(-2))));
    assert!(!pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1).sideways(1))));
}

#[test]
fn rook_moves() {
    let loc = Location::from_algebraic_notation(String::from("c6"));
    let board = Board::from_fen("7k/8/2R5/8/8/8/8/7K w - - 1 1");
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    println!("{pos_moves:?}");
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.sideways(3))));
}