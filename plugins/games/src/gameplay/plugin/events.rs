use serde::{Deserialize, Serialize};

use bevy_ecs::{
    entity::MapEntities,
    prelude::{Entity, EntityMapper, Event},
};

use chess::{actions::Action, pieces::PieceDefinition};

#[derive(Clone)]
#[derive(Event)]
#[derive(Deserialize, Serialize)]
pub struct RequestTurnEvent {
    pub game: Entity,
    pub piece: Entity,
    pub action: Action,
    pub promotion: Option<PieceDefinition>,
}

impl RequestTurnEvent {
    pub fn new(piece: Entity, game: Entity, action: Action) -> Self {
        Self {
            piece,
            game,
            action,
            promotion: None,
        }
    }

    pub fn new_with_mutation(
        piece: Entity,
        game: Entity,
        action: Action,
        promotion: PieceDefinition,
    ) -> Self {
        Self {
            piece,
            game,
            action,
            promotion: Some(promotion),
        }
    }
}

impl MapEntities for RequestTurnEvent {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.piece = mapper.map_entity(self.piece);
        self.game = mapper.map_entity(self.game);
        self.action.side_effects = self
            .action
            .side_effects
            .iter()
            .map(|(entity, data)| (mapper.map_entity(*entity), data.clone()))
            .collect();
    }
}

// A useful event for informing the controller that it must provide a mutation to continue
#[derive(Clone)]
#[derive(Event)]
#[derive(Deserialize, Serialize)]
pub struct RequireMutationEvent {
    pub piece: Entity,
    pub game: Entity,
    pub action: Action,
}

impl MapEntities for RequireMutationEvent {
    fn map_entities<M: EntityMapper>(&mut self, mapper: &mut M) {
        self.piece = mapper.map_entity(self.piece);
        self.action.side_effects = self
            .action
            .side_effects
            .iter()
            .map(|(entity, data)| (mapper.map_entity(*entity), data.clone()))
            .collect();
    }
}
