use bevy::prelude::Component;

mod king;
mod pawn;
mod piece;

pub use piece::{AdvancedBuilder, EliteBuilder, LegendaryBuilder, MajorBuilder, MinorBuilder};

// Piece identity described by the starting squares
#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub enum PieceKind {
    King,
    Piece,
    Pawn,
}
