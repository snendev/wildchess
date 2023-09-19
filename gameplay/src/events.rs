use bevy::prelude::{Entity, Event};

use chess::pieces::{Action, PieceDefinition};

#[derive(Event)]
pub struct TurnEvent {
    pub entity: Entity,
    pub action: Action,
    pub mutation: Option<PieceDefinition>,
}

impl TurnEvent {
    pub fn action(entity: Entity, action: Action) -> Self {
        TurnEvent {
            entity,
            action,
            mutation: None,
        }
    }

    pub fn mutation(entity: Entity, action: Action, mutated_piece: PieceDefinition) -> Self {
        TurnEvent {
            entity,
            action,
            mutation: Some(mutated_piece),
        }
    }
}

#[derive(Event)]
pub struct IssueMoveEvent(pub Entity, pub Action);

// A useful event for informing the controller that it must provide a mutation to continue
#[derive(Clone, Event)]
pub struct RequestMutationEvent(pub Entity, pub Action);
#[derive(Clone, Event)]
pub struct IssueMutationEvent(pub Entity, pub Action, pub PieceDefinition);
