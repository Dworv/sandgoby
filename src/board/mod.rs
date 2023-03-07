mod status;
mod step;

use crate::Piece;
pub use status::Status;
pub use step::Step;

pub struct Board<P: Piece> {
    pieces: Vec<Option<P>>,
}
