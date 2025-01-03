use bevy::app::prelude::{App, Plugin};
use bevy_replicon::prelude::AppRuleExt;

pub mod actions;
pub mod behavior;
pub mod board;
pub mod pattern;
pub mod pieces;
pub mod team;

pub struct ChessPlugin;

impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        // TODO: should be plugins for each submodule instead
        app.replicate_mapped::<actions::Actions>()
            .replicate_mapped::<actions::LastAction>()
            .replicate::<board::Board>()
            .replicate_mapped::<board::OnBoard>()
            .replicate::<pieces::Mutation>()
            .replicate::<pieces::Orientation>()
            .replicate::<pieces::PieceIdentity>()
            .replicate::<pieces::Position>()
            .replicate::<pieces::Royal>()
            .replicate::<team::Team>();
    }
}
