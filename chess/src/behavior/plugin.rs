use std::marker::PhantomData;

use bevy_app::prelude::{App, Plugin, PostUpdate};
use bevy_ecs::prelude::{
    apply_deferred, Commands, Component, Entity, IntoSystem, IntoSystemConfigs, Query, Ref,
    SystemSet, With,
};

use fairy_gameboard::GameBoard;

use crate::{
    Action,
    Actions,
    Behavior,
    Orientation,
    PatternBehavior,
    Position,
    // EnPassantBehavior, MimicBehavior, RelayBehavior,
};

use super::{
    BoardPieceCache,
    BoardThreatsCache,
    // CastlingBehavior, CastlingTarget
};

// N.B. Use this to configure run conditions so that actions are not calculated every frame
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct BehaviorsSet;

pub struct BehaviorsPlugin<Board, System, Params>
where
    Board: GameBoard,
    System: IntoSystem<(), Option<Action<Board>>, Params>,
{
    on_action: System,
    board_marker: PhantomData<Board>,
    params_marker: PhantomData<Params>,
}

impl<Board, System, Params> BehaviorsPlugin<Board, System, Params>
where
    Board: GameBoard + 'static,
    System: IntoSystem<(), Option<Action<Board>>, Params>,
{
    // TODO: remove...?
    pub fn from_input_system(input_system: System) -> Self {
        Self {
            on_action: input_system,
            board_marker: PhantomData::<Board>,
            params_marker: PhantomData::<Params>,
        }
    }

    fn clear_actions(mut piece_query: Query<&mut Actions<Board>>) {
        for mut actions in piece_query.iter_mut() {
            actions.clear();
        }
    }

    fn disable_on_move<T: Component>(
        mut commands: Commands,
        moved_piece_query: Query<(Entity, Ref<Position<Board>>, Ref<Orientation<Board>>), With<T>>,
    ) {
        for (piece, position_ref, orientation_ref) in moved_piece_query.iter() {
            if (position_ref.is_changed() && !position_ref.is_added())
                || (orientation_ref.is_changed() && !orientation_ref.is_added())
            {
                commands.entity(piece).remove::<T>();
            }
        }
    }
}

impl<Board, System, Params> Plugin for BehaviorsPlugin<Board, System, Params>
where
    Board: GameBoard + Send + Sync + 'static,
    System: IntoSystem<(), Option<Action<Board>>, Params> + Clone + Send + Sync + 'static,
    Params: Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                (Self::clear_actions, BoardPieceCache::track_pieces),
                (
                    self.on_action
                        .clone()
                        .pipe(PatternBehavior::calculate_actions_system),
                    // self.on_action
                    //     .clone()
                    //     .pipe(EnPassantBehavior::calculate_actions_system),
                    // self.on_action
                    //     .clone()
                    //     .pipe(MimicBehavior::calculate_actions_system),
                    // self.on_action
                    //     .clone()
                    //     .pipe(RelayBehavior::calculate_actions_system),
                ),
                apply_deferred,
                (
                    PatternBehavior::take_actions_system,
                    // EnPassantBehavior::take_actions_system,
                    // MimicBehavior::take_actions_system,
                    // RelayBehavior::take_actions_system,
                ),
                BoardThreatsCache::track_pieces,
                // CastlingBehavior::calculate_actions_system,
                // (
                //     Self::disable_on_move::<CastlingTarget>,
                //     Self::disable_on_move::<CastlingBehavior>,
                // ),
            )
                .chain()
                .in_set(BehaviorsSet),
        );
    }
}
