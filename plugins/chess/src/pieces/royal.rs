use serde::{Deserialize, Serialize};

use bevy::prelude::{Component, Reflect};

// Once all Royal pieces are captured, a player loses the game.
#[derive(Clone, Copy, Debug, Default)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub struct Royal;
