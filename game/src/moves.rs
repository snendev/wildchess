use bevy::prelude::{Entity, Event};

use crate::{board::PieceIdentity, Square};

pub struct Promotion {
    pub to_piece: PieceIdentity,
}

impl Promotion {
    pub fn to(piece: PieceIdentity) -> Self {
        Promotion { to_piece: piece }
    }
}

#[derive(Event)]
pub struct MovePieceEvent(pub Entity, pub Square, pub Option<Promotion>);
