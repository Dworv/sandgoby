use crate::{Board, PossibleMove, Square, Team::*};

#[test]
fn alg_in() {
    assert_eq!(Square(6i8, 4i8).in_alg(), "e2");
}

#[test]
fn alg_out() {
    assert_eq!(
        Square(1i8, 4i8),
        Square::from_alg("e7")
    )
}

#[test]
fn basic_pawn_moves() {
    let loc = Square::from_alg("e2");
    let board = Board::normal_board();
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1))));
}

#[test]
fn pawn_taking() {
    let loc = Square::from_alg("d5");
    let board = Board::from_fen("7k/8/4p3/2pP4/8/8/8/7K w - c6 1 1");
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1))));
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1).sideways(-1))));
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 1).sideways(1))));
}

#[test]
fn pawn_promotion() {
    let loc = Square::from_alg("a7");
    let board = Board::from_fen("7k/P7/8/8/8/8/8/7K w - - 1 1");
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    println!("{:?}", pos_moves);
    let mut promote = PossibleMove::new(loc, loc.forwards(White, 1));
    promote.set_promotion(true);
    assert!(pos_moves.contains(&promote));
}

#[test]
fn knight_moves() {
    let loc = Square::from_alg("c6");
    let board = Board::from_fen("7k/8/2N5/8/8/8/8/7K w - - 1 1");
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    println!("{pos_moves:?}");
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.forwards(White, 2).sideways(1))));
}

#[test]
fn bishop_moves() {
    let loc = Square::from_alg("c6");
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
    let loc = Square::from_alg("c6");
    let board = Board::from_fen("7k/8/2R5/8/8/8/8/7K w - - 1 1");
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    println!("{pos_moves:?}");
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.sideways(3))));
}

#[test]
fn queen_moves() {
    let loc = Square::from_alg("c6");
    let board = Board::from_fen("7k/8/2Q5/8/8/8/8/7K w - - 1 1");
    let pos_moves = board.get(loc).unwrap().possible_moves(loc, &board);
    println!("{pos_moves:?}");
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.sideways(3))));
    assert!(pos_moves.contains(&PossibleMove::new(loc, loc.sideways(3).forwards(White, -3))));
}