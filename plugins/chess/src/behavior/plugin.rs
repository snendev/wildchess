use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::prelude::{IntoSystemConfigs, Query, SystemSet};
use bevy_replicon::prelude::AppRuleExt;

use crate::{
    actions::Actions,
    behavior::{Behavior, EnPassantBehavior, PatternBehavior, RelayBehavior},
};

use super::{
    kinds::disable_on_move, BoardPieceCache, BoardThreatsCache, CastlingBehavior, CastlingTarget,
};

// N.B. Use this to configure run conditions so that actions are not calculated every frame
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct BehaviorsSystems;

pub struct BehaviorsPlugin;

fn clear_actions(mut piece_query: Query<&mut Actions>) {
    for mut actions in piece_query.iter_mut() {
        actions.clear();
    }
}

impl Plugin for BehaviorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (clear_actions, BoardPieceCache::track_pieces),
                (
                    PatternBehavior::calculate_actions_system,
                    EnPassantBehavior::calculate_actions_system,
                    RelayBehavior::calculate_actions_system,
                ),
                (
                    PatternBehavior::take_actions_system,
                    EnPassantBehavior::take_actions_system,
                    RelayBehavior::take_actions_system,
                ),
                BoardThreatsCache::track_pieces,
                CastlingBehavior::calculate_actions_system,
                (
                    disable_on_move::<CastlingTarget>,
                    disable_on_move::<CastlingBehavior>,
                ),
            )
                .chain()
                .in_set(BehaviorsSystems),
        );

        app.replicate::<BoardPieceCache>()
            .replicate::<BoardThreatsCache>()
            .replicate::<PatternBehavior>()
            .replicate::<CastlingBehavior>()
            .replicate::<CastlingTarget>()
            .replicate::<EnPassantBehavior>()
            .replicate::<RelayBehavior>();
    }
}
