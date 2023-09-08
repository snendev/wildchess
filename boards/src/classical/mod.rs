use bevy::prelude::{Component, Reflect, ReflectComponent};

use chess::pieces::PieceDefinition;

pub(self) mod pieces;

#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub enum ClassicalIdentity {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    #[default]
    Pawn,
}

type ClassicalPiece = PieceDefinition<ClassicalIdentity>;

mod layout;
pub(crate) use layout::ClassicalLayout;
