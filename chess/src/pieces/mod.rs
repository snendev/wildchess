use bevy_ecs::prelude::Bundle;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{actions::Actions, behavior::PieceBehaviors, team::Team};

mod identity;
pub use identity::PieceIdentity;

mod mutation;
pub use mutation::{Mutation, MutationCondition, MutationRequired};

mod orientation;
pub use orientation::Orientation;

mod position;
pub use position::Position;

mod royal;
pub use royal::Royal;

#[derive(Clone, Debug, Default)]
#[derive(Bundle)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct PieceBundle {
    pub position: Position,
    pub orientation: Orientation,
    pub team: Team,
    pub actions: Actions,
}

impl PieceBundle {
    pub fn new(start_position: Position, team: Team) -> Self {
        PieceBundle {
            position: start_position,
            orientation: team.orientation(),
            team,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct PieceDefinition {
    pub behaviors: PieceBehaviors,
    pub identity: PieceIdentity,
    pub mutation: Option<Mutation>,
    pub royal: Option<Royal>,
}

impl PieceDefinition {
    pub fn new(behaviors: PieceBehaviors, identity: PieceIdentity) -> Self {
        PieceDefinition {
            behaviors,
            identity,
            ..Default::default()
        }
    }
}
