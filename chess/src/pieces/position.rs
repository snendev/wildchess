#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use bevy_ecs::prelude::Component;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::board::{File, Rank, Square};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
