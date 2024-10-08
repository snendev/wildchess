use bevy_app::prelude::{App, Plugin};
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

        #[cfg(feature = "reflect")]
        app.register_type::<actions::Action>()
            .register_type::<actions::Actions>()
            .register_type::<actions::LastAction>()
            .register_type::<behavior::PatternBehavior>()
            .register_type::<board::Square>()
            .register_type::<board::Rank>()
            .register_type::<board::File>()
            .register_type::<board::Board>()
            .register_type::<board::OnBoard>()
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
            .register_type::<pieces::Mutation>()
            .register_type::<pieces::Orientation>()
            .register_type::<pieces::PieceIdentity>()
            .register_type::<pieces::Position>()
            .register_type::<pieces::Royal>()
            .register_type::<team::Team>();
    }
}
