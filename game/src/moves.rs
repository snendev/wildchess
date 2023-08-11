use bevy::prelude::{Entity, Event};

use crate::{pieces::PieceKind, Square};

pub struct Promotion {
    pub to_piece: PieceKind,
}

impl Promotion {
    pub fn to(piece: PieceKind) -> Self {
        Promotion { to_piece: piece }
    }
}

#[derive(Event)]
pub struct MovePieceEvent(pub Entity, pub Square, pub Option<Promotion>);
