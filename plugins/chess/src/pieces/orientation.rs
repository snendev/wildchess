use serde::{Deserialize, Serialize};

use bevy::prelude::{Component, Reflect};

// Once all Royal pieces are captured, a player loses the game.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub enum Orientation {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    pub fn flip(self) -> Self {
        match self {
            Orientation::Up => Orientation::Down,
            Orientation::Down => Orientation::Up,
            Orientation::Left => Orientation::Right,
            Orientation::Right => Orientation::Left,
        }
    }

    pub fn orient(&self, (x, y): (i16, i16)) -> (i16, i16) {
        match self {
            Orientation::Up => (x, y),
            Orientation::Down => (x, -y),
            Orientation::Left => (-y, x),
            Orientation::Right => (y, -x),
        }
    }

    pub fn other_orientations(&self) -> Vec<Orientation> {
        match self {
            Orientation::Up => vec![Orientation::Down, Orientation::Left, Orientation::Right],
            Orientation::Down => vec![Orientation::Up, Orientation::Left, Orientation::Right],
            Orientation::Left => vec![Orientation::Up, Orientation::Down, Orientation::Right],
            Orientation::Right => vec![Orientation::Up, Orientation::Down, Orientation::Left],
        }
    }
}
