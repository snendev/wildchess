use bevy::prelude::{App, Plugin};

pub mod behavior;
pub mod board;
pub mod pieces;
pub mod team;

pub struct ChessTypesPlugin;

impl Plugin for ChessTypesPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.register_type::<team::Team>()
            .register_type::<behavior::PatternBehavior>()
            .register_type::<board::Square>()
            .register_type::<board::Rank>()
            .register_type::<board::File>()
            .register_type::<board::Board>()
            .register_type::<pieces::Position>()
            .register_type::<pieces::Pattern>()
            .register_type::<pieces::Constraints>()
            .register_type::<pieces::FromRankConstraint>()
            .register_type::<pieces::ForbiddenTargetConstraint>()
            .register_type::<pieces::CaptureRules>()
            .register_type::<pieces::CaptureMode>()
            .register_type::<pieces::CapturePattern>()
            .register_type::<pieces::Scanner>()
            .register_type::<pieces::ScanMode>()
            .register_type::<pieces::Step>()
            .register_type::<pieces::RSymmetry>()
            .register_type::<pieces::ABSymmetry>()
            .register_type::<pieces::TargetKind>()
            .register_type::<pieces::Action>()
            .register_type::<pieces::Actions>();
    }
}
