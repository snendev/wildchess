use bevy::prelude::Commands;

use crate::components::{Board, PlayerBundle, Team, Turn};

pub fn add_wild_pieces(mut commands: Commands) {
    commands.spawn(Board::wild_configuration());
    commands.spawn((PlayerBundle::new(Team::White), Turn));
    commands.spawn(PlayerBundle::new(Team::Black));
}
