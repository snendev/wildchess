use bevy::prelude::{Component, Entity, Reflect};

mod game;
pub use game::{
    AntiGame, Atomic, ClockConfiguration, Crazyhouse, Game, GameBoard, GameSpawner, PieceSet,
    WinCondition,
};
mod clock;
pub use clock::Clock;

mod turns;
pub use turns::{ActionHistory, HasTurn, History, Ply};

#[derive(Component)]
pub struct Player;

#[derive(Clone, Copy, Component, Debug, Reflect)]
pub struct InGame(pub Entity);
