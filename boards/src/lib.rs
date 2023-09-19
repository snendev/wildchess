use bevy::prelude::Component;

pub mod classical;
pub mod knight_relay;
pub mod super_relay;
pub mod wild;

mod plugin;
pub use plugin::BoardPlugin;

pub(crate) mod utils;

#[derive(Clone, Component, Debug, Default)]
pub enum Game {
    Chess,
    #[default]
    WildChess,
    SuperRelayChess,
    KnightRelayChess,
    // Shogi,    // TODO
    // Checkers, // TODO
    //
}
