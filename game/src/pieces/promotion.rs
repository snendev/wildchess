use bevy::prelude::Component;

use crate::{Behavior, Rank};

#[derive(Clone, Debug, Component)]
pub struct Promotable {
    // the Rank required to reach promotion r
    // flipped if Team::Black
    pub local_rank: Rank,
    // the upgraded Behaviors to choose from
    pub behaviors: Vec<Behavior>,
}
