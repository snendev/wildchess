use serde::{Deserialize, Serialize};

use bevy::prelude::{Component, Reflect};

use crate::pieces::Orientation;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub enum Team {
    #[default]
    White,
    Black,
}

impl Team {
    pub fn orientation(self) -> Orientation {
        match self {
            Team::White => Orientation::Up,
            Team::Black => Orientation::Down,
        }
    }

    pub fn get_next(self) -> Self {
        match self {
            Team::White => Team::Black,
            Team::Black => Team::White,
        }
    }

    pub fn name(self) -> String {
        format!("{self:?}")
    }

    pub fn code(self) -> char {
        match self {
            Team::White => 'w',
            Team::Black => 'b',
        }
    }
}
