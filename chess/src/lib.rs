use bevy_app::prelude::{App, Plugin};

pub mod actions;
pub mod behavior;
pub mod board;
pub mod pattern;
pub mod pieces;
pub mod team;

pub struct ChessTypesPlugin;

impl Plugin for ChessTypesPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "reflect")]
        app.register_type::<team::Team>()
            .register_type::<behavior::PatternBehavior>()
            .register_type::<board::Square>()
            .register_type::<board::Rank>()
            .register_type::<board::File>()
            .register_type::<board::Board>()
            .register_type::<pattern::Pattern>()
            .register_type::<pattern::Constraints>()
            .register_type::<pattern::FromRankConstraint>()
            .register_type::<pattern::ForbiddenTargetConstraint>()
            .register_type::<pattern::CaptureRules>()
            .register_type::<pattern::CaptureMode>()
            .register_type::<pattern::CapturePattern>()
            .register_type::<pattern::Scanner>()
            .register_type::<pattern::ScanMode>()
            .register_type::<pattern::Step>()
            .register_type::<pattern::RSymmetry>()
            .register_type::<pattern::ABSymmetry>()
            .register_type::<pattern::TargetKind>()
            .register_type::<pieces::Position>()
            .register_type::<actions::Action>()
            .register_type::<actions::Actions>();
    }
}
