use bevy::prelude::Commands;

use crate::components::Board;

pub fn add_wild_pieces(mut commands: Commands) {
    commands.spawn(Board::wild_configuration());
}
