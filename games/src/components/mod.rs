#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use bevy_ecs::prelude::{Component, Entity};
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::Reflect;

mod game;
pub use game::{
    AntiGame, Atomic, ClockConfiguration, Crazyhouse, Game, GameBoard, GameSpawner, WinCondition,
};
mod clock;
pub use clock::Clock;

mod turns;
pub use turns::{ActionHistory, HasTurn, History, Ply};

#[derive(Component)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Player;

#[derive(Clone, Copy, Component, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct InGame(pub Entity);
