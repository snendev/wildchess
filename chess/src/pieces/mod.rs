use bevy::prelude::{Bundle, Reflect};

use fairy_gameboard::GameBoard;

use crate::{actions::Actions, team::Team};

mod identity;
pub use identity::PieceIdentity;

mod mutation;
pub use mutation::{Mutation, MutationCondition, MutationRequired};

mod position;
pub use position::{Position, Orientation};

mod royal;
pub use royal::Royal;

#[derive(Clone, Debug, Default, Bundle, Reflect)]
pub struct PieceBundle<B: GameBoard + 'static> {
    pub position: Position<B>,
    pub orientation: Orientation<B>,
    pub actions: Actions<B>,
    pub team: Team,
}

impl <B: GameBoard>PieceBundle<B> {
    pub fn new(start_position: B::Vector, team: Team) -> Self {
        PieceBundle {
            position: start_position,
            orientation: team.orientation(),
            team,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Reflect)]
pub struct PieceDefinition<B: GameBoard> {
    pub behaviors: PieceBehaviors,
    pub identity: PieceIdentity,
    pub mutation: Option<Mutation<B>>,
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
