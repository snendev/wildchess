use bevy::{prelude::Component, reflect::Reflect};

use super::Square;

#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
pub struct Board {
    pub size: Square,
}
