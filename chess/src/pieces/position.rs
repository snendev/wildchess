use bevy::prelude::{Component, Reflect, ReflectComponent};

use crate::square::{File, Rank, Square};

#[derive(Clone, Component, Debug, Default, PartialEq, Eq, Reflect)]
#[reflect(Component)]
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
