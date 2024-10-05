use serde::{Deserialize, Serialize};

use bevy::prelude::{Component, Reflect};

// A name for the "kind" of piece this is. Usually relates to a specific set of behaviors,
// but variants often change these.
// It is mostly useful for supplying contextual information to users, such as displaying a
// particular icon.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Component, Reflect)]
#[derive(Deserialize, Serialize)]
pub enum PieceIdentity {
    // The default: implies that this piece does not fit an existing stereotype
    // #[default]
    // Wild,
    // Somewhat universal
    King,
    // Chess
    Queen,
    Rook,
    Bishop,
    Knight,
    #[default]
    Pawn,
    // TODO: Shogi
    // TODO: Xiangqi
    // TODO: others
}

impl PieceIdentity {
    pub fn name(self) -> String {
        format!("{:?}", self)
    }

    pub fn code(self) -> char {
        match self {
            PieceIdentity::King => 'K',
            PieceIdentity::Queen => 'Q',
            PieceIdentity::Rook => 'R',
            PieceIdentity::Bishop => 'B',
            PieceIdentity::Knight => 'N',
            PieceIdentity::Pawn => 'P',
        }
    }
}
