use bevy::prelude::{Bundle, Component};

mod clock;
pub use clock::Clock;

mod turn;
pub use turn::Turn;

#[derive(Component)]
pub struct Player;
