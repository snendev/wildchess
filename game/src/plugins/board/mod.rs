use bevy::prelude::{App, Plugin, Startup};

mod settings;
use settings::add_wild_pieces;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_wild_pieces);
    }
}
