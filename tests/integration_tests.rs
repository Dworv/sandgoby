use sandgoby;

/*
#[ignore = "not done yet"]
#[test]
fn basic_test() {
    let board: sandgoby::Board<sandgoby::ClassicPieces> = sandgoby::Board::assemble(
        Fen::compile("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap(),
        |square| square.0 < 8 && square.0 >= 0 && square.1 < 8 && square.1 >= 0
    ).unwrap();
    board.get_status();
    let mut moves = board.get_moves();
    board.make_move(moves.next());
}
*/
