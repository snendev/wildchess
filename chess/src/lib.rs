use bevy::prelude::{App, Plugin};

pub mod pieces;
pub mod square;
pub mod team;

pub struct ChessTypesPlugin;

impl Plugin for ChessTypesPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.register_type::<team::Team>()
            .register_type::<square::Square>()
            .register_type::<square::Rank>()
            .register_type::<square::File>()
            .register_type::<pieces::Position>()
            .register_type::<pieces::Behavior>()
            .register_type::<pieces::Pattern>()
            .register_type::<pieces::PatternStep>()
            .register_type::<pieces::TargetMode>()
            .register_type::<pieces::SearchMode>()
            .register_type::<pieces::Targets>();
    }
}
