use bevy::prelude::{Entity, Event};

use chess::{actions::Action, pieces::PieceDefinition, team::Team};

#[derive(Event)]
pub struct TurnEvent {
    pub piece: Entity,
    pub board: Entity,
    pub action: Action,
    pub mutation: Option<PieceDefinition>,
}

impl TurnEvent {
    pub fn action(piece: Entity, board: Entity, action: Action) -> Self {
        TurnEvent {
            piece,
            board,
            action,
            mutation: None,
        }
    }

    pub fn mutation(
        piece: Entity,
        board: Entity,
        action: Action,
        mutated_piece: PieceDefinition,
    ) -> Self {
        TurnEvent {
            piece,
            board,
            action,
            mutation: Some(mutated_piece),
        }
    }
}

#[derive(Event)]
pub struct IssueMoveEvent {
    pub piece: Entity,
    pub board: Entity,
    pub action: Action,
}

// A useful event for informing the controller that it must provide a mutation to continue
#[derive(Clone, Event)]
pub struct RequestMutationEvent {
    pub piece: Entity,
    pub board: Entity,
    pub action: Action,
}

#[derive(Clone, Event)]
pub struct IssueMutationEvent {
    pub piece: Entity,
    pub board: Entity,
    pub action: Action,
    pub piece_definition: PieceDefinition,
}

#[derive(Clone, Event)]
pub struct GameoverEvent {
    pub winner: Team,
}
