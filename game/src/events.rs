use bevy::prelude::{Entity, Event};

use crate::{components::Behavior, Square};

#[derive(Clone)]
pub struct Movement {
    pub entity: Entity,
    pub target_square: Square,
}

#[derive(Event)]
pub enum TurnEvent {
    Movement(Movement),
    Promotion(Movement, Behavior),
}

#[derive(Event)]
pub struct IssueMoveEvent(pub Movement);

// A useful event for informing the controller that it must provide a promotion to continue
#[derive(Clone, Event)]
pub struct RequestPromotionEvent(pub Movement);
#[derive(Clone, Event)]
pub struct IssuePromotionEvent(pub Movement, pub Behavior);
