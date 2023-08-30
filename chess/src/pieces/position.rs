use bevy::prelude::Component;

use crate::{
    square::{LocalSquare, Square},
    team::Team,
};

#[derive(Clone, Component, Debug, PartialEq, Eq)]
pub struct Position(pub Square);

#[derive(Clone, Component, Debug, PartialEq, Eq, Hash)]
pub struct StartPosition(pub LocalSquare);

impl StartPosition {
    pub fn to_position(&self, team: &Team) -> Position {
        Position(self.0.to_square(team))
    }
}
