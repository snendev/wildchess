use bevy::prelude::{Component, Entity};

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

// TODO: use this
#[derive(Clone, Copy, Component, Debug)]
pub struct InGame(Entity);
