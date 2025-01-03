use serde::{Deserialize, Serialize};

use bevy::{
    ecs::entity::MapEntities,
    prelude::{Bundle, Component, Entity, EntityMapper},
};

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
#[derive(Deserialize, Serialize)]
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

#[derive(Clone, Debug)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
pub struct HasPieces(pub Vec<Entity>);

impl MapEntities for HasPieces {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.0 = self
            .0
            .iter()
            .map(|entity| mapper.map_entity(*entity))
            .collect();
    }
}
