use bevy::prelude::{Bundle, Component, Resource};

mod king;
mod pawn;
mod piece;
mod promotion;

pub use piece::{AdvancedBuilder, EliteBuilder, LegendaryBuilder, MajorBuilder, MinorBuilder};

use crate::{Behavior, Square, Team, Vision};

// Piece identity described by the starting squares
#[derive(Clone, Copy, Component, Debug, PartialEq)]
pub enum PieceKind {
    SquareAH,
    SquareBG,
    SquareCF,
    SquareD,
    King,
    Pawn,
}

#[derive(Clone, Bundle)]
pub struct PieceBundle {
    pub behavior: Behavior,
    pub square: Square,
    pub team: Team,
    pub vision: Vision,
    pub kind: PieceKind,
}

impl PieceBundle {
    pub fn new(kind: PieceKind, behavior: Behavior, team: Team, square: Square) -> Self {
        PieceBundle {
            behavior,
            square,
            team,
            vision: Vision::default(),
            kind,
        }
    }

    pub fn from_configuration(
        PieceConfiguration {
            kind,
            behavior,
            starting_square,
        }: PieceConfiguration,
        team: Team,
    ) -> Self {
        PieceBundle::new(kind, behavior, team, starting_square)
    }
}

#[derive(Clone, Debug)]
pub struct PieceConfiguration {
    pub kind: PieceKind,
    pub behavior: Behavior,
    pub starting_square: Square,
}

#[derive(Resource)]
pub struct GamePieces(pub Vec<PieceConfiguration>);
