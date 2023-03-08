use crate::Square;

#[test]
fn from_alg_valid_normal() {
    let square = Square::try_from_alg("g3", (8,8)).unwrap();
    println!("square");
    assert_eq!(square, Square(5, 6));
}

#[test]
#[should_panic]
fn from_alg_invalid_len() {
    Square::try_from_alg(":", (8,8)).unwrap();
}

#[test]
#[should_panic]
fn from_alg_invalid_order() {
    Square::try_from_alg("4b", (8,8)).unwrap();
}