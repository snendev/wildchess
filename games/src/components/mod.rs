use bevy::prelude::{Component, Entity};

mod game;
pub use game::{
    AntiGame, Atomic, ClockConfiguration, Crazyhouse, GameBoard, GameSpawner, WinCondition,
};

mod player;
pub use player::{Clock, Player, Turn};

#[derive(Clone, Copy, Component, Debug)]
pub struct InGame(Entity);
