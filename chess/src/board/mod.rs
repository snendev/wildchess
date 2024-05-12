use serde::{Deserialize, Serialize};

#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::prelude::{Component, Entity};
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

mod square;
pub use square::{File, Rank, Square};

#[derive(Clone, Copy, Debug)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]

pub struct OnBoard(pub Entity);

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
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

    pub fn scan(&self, origin: Square, scan_vector: (i16, i16)) -> BoardIterator<'_> {
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
