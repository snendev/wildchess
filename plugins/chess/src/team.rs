use serde::{Deserialize, Serialize};

use bevy_ecs::prelude::Component;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::Reflect;

use crate::pieces::Orientation;

#[derive(Clone, Copy, Component, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
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

    pub fn get_next(&self) -> Self {
        match self {
            Team::White => Team::Black,
            Team::Black => Team::White,
        }
    }
}
