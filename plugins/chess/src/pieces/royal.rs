use serde::{Deserialize, Serialize};

use bevy::prelude::{Component};

// Once all Royal pieces are captured, a player loses the game.
#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
pub struct Royal;
