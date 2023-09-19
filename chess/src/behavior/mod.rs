use bevy::{
    prelude::{Component, In, Query},
    reflect::Reflect,
};

use crate::{
    board::Board,
    pieces::{Action, Actions, Orientation, Position},
    team::Team,
};

mod pattern;
pub use pattern::PatternBehavior;

mod en_passant;
pub use en_passant::EnPassantBehavior;

mod mimic;
pub use mimic::MimicBehavior;

mod relay;
pub use relay::RelayBehavior;

// TODO:
// mod castling;
// mod mirror;
// mod rotation;

pub trait Behavior {
    fn add_actions_system(
        last_action: In<Option<Action>>,
        board_query: Query<&Board>,
        piece_query: Query<(Option<&Self>, &Position, &Orientation, &Team, &mut Actions)>,
    ) where
        Self: Component + Sized;
}

#[derive(Clone, Debug, Default, Reflect)]
pub struct PieceBehaviors {
    pub pattern: Option<PatternBehavior>,
    pub en_passant: Option<EnPassantBehavior>,
    pub mimic: Option<MimicBehavior>,
    pub relay: Option<RelayBehavior>,
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

impl From<MimicBehavior> for PieceBehaviors {
    fn from(behavior: MimicBehavior) -> Self {
        PieceBehaviors {
            mimic: Some(behavior),
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
