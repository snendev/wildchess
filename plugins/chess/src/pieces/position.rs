use serde::{Deserialize, Serialize};

use bevy::prelude::{Component};

use crate::board::{File, Rank, Square};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
pub struct Position(pub Square);

impl From<Square> for Position {
    fn from(square: Square) -> Self {
        Position(square)
    }
}

impl From<(File, Rank)> for Position {
    fn from(square: (File, Rank)) -> Self {
        Square::from(square).into()
    }
}
