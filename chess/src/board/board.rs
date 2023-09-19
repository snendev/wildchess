use bevy::{prelude::Component, reflect::Reflect};

use super::{File, Rank, Square};

#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
pub struct Board {
    pub size: Square,
}

impl Board {
    pub fn chess_board() -> Self {
        Board {
            size: Square::new(File::H, Rank::EIGHT),
        }
    }

    pub fn shogi_board() -> Self {
        Board {
            size: Square::new(File(8), Rank(8)),
        }
    }

    pub fn scan<'a>(&'a self, origin: Square, scan_vector: (i16, i16)) -> BoardIterator<'a> {
        BoardIterator {
            board: self,
            current_square: origin,
            scan_vector,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoardIterator<'a> {
    board: &'a Board,
    current_square: Square,
    scan_vector: (i16, i16),
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let next_square = self.current_square.checked_add(
            self.scan_vector.0,
            self.scan_vector.1,
            &self.board.size,
        );
        if let Some(square) = next_square {
            self.current_square = square;
        }
        next_square
    }
}
