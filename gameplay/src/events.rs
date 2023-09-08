use bevy::prelude::{Entity, Event};

use chess::{
    pieces::{Behavior, PieceDefinition},
    square::Square,
};

#[derive(Clone)]
pub struct Movement {
    pub entity: Entity,
    pub target_square: Square,
}

#[derive(Event)]
pub enum TurnEvent {
    Movement(Movement),
    Mutation(Movement, PieceDefinition),
}

#[derive(Event)]
pub struct IssueMoveEvent(pub Movement);

// A useful event for informing the controller that it must provide a promotion to continue
#[derive(Clone, Event)]
pub struct RequestMutationEvent(pub Movement);
#[derive(Clone, Event)]
pub struct IssueMutationEvent(pub Movement, pub PieceDefinition);
