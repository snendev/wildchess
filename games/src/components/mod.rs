use bevy::prelude::{Component, Entity, Reflect};

mod game;
pub use game::{
    AntiGame, Atomic, ClockConfiguration, Crazyhouse, GameBoard, GameSpawner, WinCondition,
};
mod clock;
pub use clock::Clock;

mod turn;
pub use turn::Turn;

#[derive(Component)]
pub struct Player;

#[derive(Clone, Copy, Component, Debug, Reflect)]
pub struct InGame(pub Entity);
