use serde::{Deserialize, Serialize};

use bevy::prelude::Reflect;

use chess::{board::Square, pieces::PieceDefinition};

mod classical;
pub use classical::ClassicalLayout;
mod knight_relay;
pub use knight_relay::KnightRelayLayout;
mod super_relay;
pub use super_relay::SuperRelayLayout;
mod wild;
pub use wild::{ClassicWildLayout, FeaturedWildLayout, RandomWildLayout, WildPieceSet};

// Defines how to position a piece relative to a player's starting orientation
#[derive(Clone, Debug, Default)]
#[derive(Reflect)]
#[derive(Deserialize, Serialize)]
pub struct PieceSpecification {
    pub piece: PieceDefinition,
    pub start_square: Square,
}

impl PieceSpecification {
    pub fn new(piece: PieceDefinition, start_square: Square) -> Self {
        Self {
            piece,
            start_square,
        }
    }
}
