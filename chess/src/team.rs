use bevy::prelude::{Component, Reflect, ReflectComponent};

use crate::pieces::Orientation;

#[derive(Clone, Copy, Component, Debug, Default, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub enum Team {
    #[default]
    White,
    Black,
}

impl Team {
    pub fn orientation(&self) -> Orientation {
        match self {
            Team::White => Orientation::Up,
            Team::Black => Orientation::Down,
        }
    }
}
