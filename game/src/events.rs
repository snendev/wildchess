use bevy::prelude::{Entity, Event};

use crate::{components::Behavior, Square};

pub struct Movement {
    pub entity: Entity,
    pub target_square: Square,
}

pub struct Promotion {
    pub entity: Entity,
    pub behavior: Behavior,
}

#[derive(Clone)]
pub struct RequestPromotion {
    entity: Entity,
}

impl RequestPromotion {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn promote(&self, behavior: Behavior) -> Promotion {
        Promotion {
            entity: self.entity,
            behavior,
        }
    }
}

// N.B. TypeState pattern enforces transitions between PieceEvent<Move>, PieceEvent<RequestPromotion>, and PieceEvent<Promotion>
#[derive(Event)]
pub struct PieceEvent<T>(T);

impl<T> PieceEvent<T> {
    pub fn get<'a>(&'a self) -> &'a T {
        &self.0
    }
}

impl PieceEvent<Movement> {
    pub fn new(entity: Entity, target_square: Square) -> Self {
        PieceEvent(Movement {
            entity,
            target_square,
        })
    }

    pub fn to_promotion(&self, behavior: Behavior) -> PieceEvent<Promotion> {
        PieceEvent(Promotion {
            entity: self.0.entity,
            behavior,
        })
    }
}

impl From<&PieceEvent<Movement>> for PieceEvent<RequestPromotion> {
    fn from(event: &PieceEvent<Movement>) -> Self {
        PieceEvent(RequestPromotion {
            entity: event.0.entity,
        })
    }
}

impl PieceEvent<Promotion> {
    pub fn new(promotion: Promotion) -> Self {
        PieceEvent(promotion)
    }
}
