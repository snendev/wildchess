use serde::{Deserialize, Serialize};

use bevy_ecs::{
    entity::MapEntities,
    prelude::{Entity, EntityMapper, Event},
};

use chess::{actions::Action, pieces::PieceDefinition, team::Team};

use crate::components::Ply;

#[derive(Debug)]
#[derive(Event)]
#[derive(Deserialize, Serialize)]
pub struct TurnEvent {
    pub ply: Ply,
    pub piece: Entity,
    pub board: Entity,
    pub game: Entity,
    pub action: Action,
    pub mutation: Option<PieceDefinition>,
}

impl TurnEvent {
    pub fn action(ply: Ply, piece: Entity, board: Entity, game: Entity, action: Action) -> Self {
        TurnEvent {
            ply,
            piece,
            board,
            game,
            action,
            mutation: None,
        }
    }

    pub fn mutation(
        ply: Ply,
        piece: Entity,
        board: Entity,
        game: Entity,
        action: Action,
        mutated_piece: PieceDefinition,
    ) -> Self {
        TurnEvent {
            ply,
            piece,
            board,
            game,
            action,
            mutation: Some(mutated_piece),
        }
    }
}

#[derive(Clone)]
#[derive(Event)]
#[derive(Deserialize, Serialize)]
pub struct RequestTurnEvent {
    pub piece: Entity,
    pub action: Action,
    pub promotion: Option<PieceDefinition>,
}

impl RequestTurnEvent {
    pub fn new(piece: Entity, action: Action) -> Self {
        Self {
            piece,
            action,
            promotion: None,
        }
    }

    pub fn new_with_mutation(piece: Entity, action: Action, promotion: PieceDefinition) -> Self {
        Self {
            piece,
            action,
            promotion: Some(promotion),
        }
    }
}

impl MapEntities for RequestTurnEvent {
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

// A useful event for informing the controller that it must provide a mutation to continue
#[derive(Clone)]
#[derive(Event)]
#[derive(Deserialize, Serialize)]
pub struct RequireMutationEvent {
    pub piece: Entity,
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
