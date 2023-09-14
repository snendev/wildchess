use bevy::prelude::{Entity, Event};

use chess::pieces::{Action, PieceDefinition};

#[derive(Event)]
pub enum TurnEvent {
    Action(Entity, Action),
    Mutation(Entity, Action, PieceDefinition),
}

#[derive(Event)]
pub struct IssueMoveEvent(pub Entity, pub Action);

// A useful event for informing the controller that it must provide a mutation to continue
#[derive(Clone, Event)]
pub struct RequestMutationEvent(pub Entity, pub Action);
#[derive(Clone, Event)]
pub struct IssueMutationEvent(pub Entity, pub Action, pub PieceDefinition);
