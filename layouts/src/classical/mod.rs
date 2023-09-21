use bevy::prelude::{Component, Reflect, ReflectComponent};

pub(crate) mod pieces;

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

mod layout;
pub use layout::ClassicalLayout;
