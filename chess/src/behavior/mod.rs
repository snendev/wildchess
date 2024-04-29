use bevy_ecs::prelude::{Bundle, Commands, Component, Entity, In, Query};
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use fairy_gameboard::GameBoard;

use crate::{
    actions::{Action, Actions},
    pieces::{Orientation, Position},
    team::Team,
    ChessBoard,
};

pub mod caches;
pub use caches::{BoardPieceCache, BoardThreat, BoardThreatsCache};

mod kinds;
pub use kinds::{
    // CastlingBehavior,
    // CastlingTarget,
    PatternBehavior,
    // EnPassantBehavior, MimicBehavior, RelayBehavior,
};

mod plugin;
pub use plugin::{BehaviorsPlugin, BehaviorsSet};

pub trait Behavior<B>
where
    B: GameBoard + Send + Sync + 'static,
{
    // Each behavior supplies is own sink for calculating actions.
    // This enables parallelizing these calculations since we don't need
    // N exclusive references to `Actions`.
    type ActionsCache: Component + From<Actions<B>> + Into<Actions<B>>;

    // All Behaviors register this system in the first "bucket".
    // It calculates the available `Actions` for each piece and stores that in its
    // `Self::ActionsCache` sink component.
    // Be sure to clear the cache each time this system is run.
    #[allow(clippy::type_complexity)]
    fn calculate_actions_system(
        commands: Commands,
        board_query: Query<(&ChessBoard<B>, &BoardPieceCache<B>)>,
        piece_query: Query<(
            Entity,
            Option<&Self>,
            Option<&mut Self::ActionsCache>,
            &Position<B>,
            &Orientation<B>,
            &Team,
        )>,
    ) where
        Self: Component + Sized;

    // All Behaviors register this system subsequent to the bucket containing all
    // `calculate_actions_system`s.
    // It takes the cached value from `Self::ActionsCache` and extends `Actions` with it.
    // These generally should be ordered.
    // TODO: Allow multiple actions on one square.
    fn take_actions_system(mut piece_query: Query<(&Self::ActionsCache, &mut Actions<B>)>)
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
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct PieceBehaviors<B: GameBoard> {
    pub pattern: Option<PatternBehavior<B>>,
    // pub en_passant: Option<EnPassantBehavior>,
    // pub mimic: Option<MimicBehavior>,
    // pub relay: Option<RelayBehavior>,
    // pub castling: Option<CastlingBehavior>,
    // pub castling_target: Option<CastlingTarget>,
}

impl<B: GameBoard> From<PatternBehavior<B>> for PieceBehaviors<B> {
    fn from(behavior: PatternBehavior<B>) -> Self {
        PieceBehaviors {
            pattern: Some(behavior),
            ..Default::default()
        }
    }
}

// impl From<EnPassantBehavior> for PieceBehaviors {
//     fn from(behavior: EnPassantBehavior) -> Self {
//         PieceBehaviors {
//             en_passant: Some(behavior),
//             ..Default::default()
//         }
//     }
// }

// impl From<MimicBehavior> for PieceBehaviors {
//     fn from(behavior: MimicBehavior) -> Self {
//         PieceBehaviors {
//             mimic: Some(behavior),
//             ..Default::default()
//         }
//     }
// }

// impl From<RelayBehavior> for PieceBehaviors {
//     fn from(behavior: RelayBehavior) -> Self {
//         PieceBehaviors {
//             relay: Some(behavior),
//             ..Default::default()
//         }
//     }
// }

// Rarely will a piece need all these behaviors
// However, this bundle is useful for calling EntityMut::remove()
// to remove all behaviors at once
#[derive(Clone, Debug, Default, Bundle)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct PieceBehaviorsBundle<B>
where
    B: GameBoard + Send + Sync + 'static,
{
    pub pattern: PatternBehavior<B>,
    // pub en_passant: EnPassantBehavior,
    // pub mimic: MimicBehavior,
    // pub relay: RelayBehavior,
}
