use bevy::prelude::Component;

use crate::{components::Behavior, Rank};

#[derive(Clone, Debug, Component, PartialEq, Hash)]
pub struct Promotable {
    // the Rank required to reach promotion
    // flipped if Team::Black
    pub local_rank: Rank,
    // the upgraded Behaviors to choose from
    pub behaviors: Vec<Behavior>,
}
