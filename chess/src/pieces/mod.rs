use bevy_ecs::prelude::Bundle;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use fairy_gameboard::GameBoard;

use crate::{actions::Actions, team::Team, PieceBehaviors};

mod identity;
pub use identity::PieceIdentity;

mod mutation;
pub use mutation::{Mutation, MutationCondition, MutationRequired};

mod position;
pub use position::{Orientation, Position};

mod royal;
pub use royal::Royal;

#[derive(Clone, Debug, Default)]
#[derive(Bundle)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct PieceBundle<B: GameBoard + 'static> {
    pub position: Position<B>,
    pub orientation: Orientation<B>,
    pub actions: Actions<B>,
    pub team: Team,
}

impl<B: GameBoard> PieceBundle<B> {
    pub fn new(start_position: B::Vector, team: Team) -> Self {
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
pub struct PieceDefinition<B: GameBoard> {
    pub behaviors: PieceBehaviors<B>,
    pub identity: PieceIdentity,
    pub mutation: Option<Mutation<B>>,
    pub royal: Option<Royal>,
}

impl<B: GameBoard> PieceDefinition<B> {
    pub fn new(behaviors: PieceBehaviors<B>, identity: PieceIdentity) -> Self {
        PieceDefinition {
            behaviors,
            identity,
            ..Default::default()
        }
    }
}
