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
pub struct Player;

#[derive(Clone, Copy, Component, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct InGame(pub Entity);
