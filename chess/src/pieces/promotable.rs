use bevy::prelude::Component;

use crate::{pieces::Behavior, square::Rank};

#[derive(Clone, Debug, Component, PartialEq, Hash)]
pub struct Promotable {
    // the Rank required to reach promotion
    // flipped if Team::Black
    pub ranks: Vec<Rank>,
    // the upgraded Behaviors to choose from
    pub behaviors: Vec<Behavior>,
}
