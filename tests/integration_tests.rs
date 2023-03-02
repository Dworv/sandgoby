use sandgoby;

#[test]
fn basic_test() {
    let board = sandgoby::Board::starting_board();
    let moves = board.possible_moves();
    let mut total = 0;
    for i in moves {
        total += dbg!(i.1).len();
    }
    assert_eq!(total, 20);
}