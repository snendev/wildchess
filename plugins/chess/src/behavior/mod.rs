use serde::{Deserialize, Serialize};

use bevy::prelude::{Bundle, Commands, Component, Entity, Query, Reflect};

use crate::{
    actions::{Actions, LastAction},
    board::{Board, OnBoard},
    pieces::{Orientation, Position},
    team::Team,
};

pub mod caches;
pub use caches::{BoardPieceCache, BoardThreat, BoardThreatsCache};

mod kinds;
pub use kinds::{
    CastlingBehavior, CastlingTarget, EnPassantBehavior, PatternBehavior, RelayBehavior,
};

mod plugin;
pub use plugin::{BehaviorsPlugin, BehaviorsSystems};

pub trait Behavior {
    // Each behavior supplies is own sink for calculating actions.
    // This enables parallelizing these calculations since we don't need
    // N exclusive references to `Actions`.
    type ActionsCache: Component + From<Actions> + Into<Actions>;

    // All Behaviors register this system in the first "bucket".
    // It calculates the available `Actions` for each piece and stores that in its
    // `Self::ActionsCache` sink component.
    // Be sure to clear the cache each time this system is run.
    #[allow(clippy::type_complexity)]
    fn calculate_actions_system(
        commands: Commands,
        board_query: Query<(Entity, &Board, &BoardPieceCache, Option<&LastAction>)>,
        piece_query: Query<(
            Entity,
            Option<&Self>,
            Option<&mut Self::ActionsCache>,
            &Position,
            &Orientation,
            &Team,
            &OnBoard,
        )>,
    ) where
        Self: Component + Sized;

    // All Behaviors register this system subsequent to the bucket containing all
    // `calculate_actions_system`s.
    // It takes the cached value from `Self::ActionsCache` and extends `Actions` with it.
    // These generally should be ordered.
    // TODO: Allow multiple actions on one square.
    fn take_actions_system(mut piece_query: Query<(&Self::ActionsCache, &mut Actions)>)
    where
        Self: Component + Sized,
        Self::ActionsCache: Clone,
    {
        for (cache, mut actions) in piece_query.iter_mut() {
            actions.extend(cache.clone().into());
        }
    }
}

// TODO: Change add_actions_system to some generate_actions that adds to an internal buffer.
// Additionally, add a take_actions fn that returns the buffered actions and clears the Behavior.
// This allows parallel system performance:
//  - Define a generic system that buffers the data for a system. Run these systems first, in
//    parallel.
//  - Add a unifying `merge_actions` system that pulls from all the Behaviors and calls
//    `take_actions` to populate Actions

// fn add_actions_system<C: Component + Behavior>(
//     In(last_action): In<Option<Action>>,
//     board_query: Query<&Board>,
//     piece_query: Query<(Option<&mut C>, &Position, &Orientation, &Team)>,
// ) {}

#[derive(Clone, Debug, Default)]
#[derive(Reflect)]
#[derive(Deserialize, Serialize)]
pub struct PieceBehaviors {
    pub pattern: Option<PatternBehavior>,
    pub en_passant: Option<EnPassantBehavior>,
    pub relay: Option<RelayBehavior>,
    pub castling: Option<CastlingBehavior>,
    pub castling_target: Option<CastlingTarget>,
}

impl From<PatternBehavior> for PieceBehaviors {
    fn from(behavior: PatternBehavior) -> Self {
        PieceBehaviors {
            pattern: Some(behavior),
            ..Default::default()
        }
    }
}

impl From<EnPassantBehavior> for PieceBehaviors {
    fn from(behavior: EnPassantBehavior) -> Self {
        PieceBehaviors {
            en_passant: Some(behavior),
            ..Default::default()
        }
    }
}

impl From<RelayBehavior> for PieceBehaviors {
    fn from(behavior: RelayBehavior) -> Self {
        PieceBehaviors {
            relay: Some(behavior),
            ..Default::default()
        }
    }
}

// Rarely will a piece need all these behaviors
// However, this bundle is useful for calling EntityMut::remove()
// to remove all behaviors at once
#[derive(Clone, Debug, Default)]
#[derive(Bundle)]
pub struct PieceBehaviorsBundle {
    pub pattern: PatternBehavior,
    pub en_passant: EnPassantBehavior,
    pub relay: RelayBehavior,
}
