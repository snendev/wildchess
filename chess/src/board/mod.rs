mod board;
pub use board::Board;

mod square;
pub use square::{File, Rank, Square};

pub mod common {
    use super::*;

    pub fn chess_board() -> Board {
        Board {
            size: Square::new(File::H, Rank::EIGHT),
        }
    }

    pub fn shogi_board() -> Board {
        Board {
            size: Square::new(File(8), Rank(8)),
        }
    }
}
