use bevy::prelude::Bundle;

use crate::components::Team;

mod behavior;
pub use behavior::{Behavior, Pattern, PatternStep, SearchMode, TargetMode};

mod kind;
pub use kind::PieceKind;

mod promotable;
pub use promotable::Promotable;

mod position;
pub use position::{Position, StartPosition};

mod targets;
pub use targets::Targets;

#[derive(Clone, Bundle)]
pub struct PieceBundle {
    pub behavior: Behavior,
    pub position: Position,
    pub start_position: StartPosition,
    pub team: Team,
    pub targets: Targets,
    pub kind: PieceKind,
}

impl PieceBundle {
    pub fn new(
        kind: PieceKind,
        behavior: Behavior,
        team: Team,
        position: Position,
        start_position: StartPosition,
    ) -> Self {
        PieceBundle {
            behavior,
            position,
            start_position,
            team,
            targets: Targets::default(),
            kind,
        }
    }

    pub fn from_configuration(
        PieceConfiguration { kind, behavior, .. }: &PieceConfiguration,
        start_position: StartPosition,
        team: Team,
    ) -> Self {
        PieceBundle::new(
            *kind,
            behavior.clone(),
            team,
            start_position.to_position(&team),
            start_position,
        )
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct PieceConfiguration {
    pub kind: PieceKind,
    pub behavior: Behavior,
    pub promotable: Option<Promotable>,
}
