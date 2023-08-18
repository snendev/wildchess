use bevy::prelude::Resource;

pub mod components;
use components::{Behavior, PieceKind, Promotable, StartPosition};

mod events;
pub use events::{Movement, PieceEvent, Promotion, RequestPromotion};

mod gameplay;
pub use gameplay::{GameplayPlugin, WildBoardPlugin};

mod square;
pub use square::{File, LocalSquare, Rank, Square};

#[derive(Clone, Debug)]
pub struct PieceConfiguration {
    pub kind: PieceKind,
    pub behavior: Behavior,
    pub promotable: Option<Promotable>,
}

#[derive(Resource)]
pub struct GamePieces(pub Vec<(PieceConfiguration, Vec<StartPosition>)>);
