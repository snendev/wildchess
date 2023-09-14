use bevy::prelude::{Component, In, Query};

use crate::{
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
// TODO: define a useful trait for coordinating Behavior systems
pub trait Behavior {
    fn add_actions_system(
        last_action: In<Option<Action>>,
        piece_query: Query<(Option<&Self>, &Position, &Orientation, &Team, &mut Actions)>,
    ) where
        Self: Component + Sized;
}
