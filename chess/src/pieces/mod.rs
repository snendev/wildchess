use bevy::prelude::{Bundle, Component};

use crate::team::Team;

mod behavior;
pub use behavior::{Behavior, Pattern, PatternStep, SearchMode, TargetMode};

mod promotable;
pub use promotable::Promotable;

mod position;
pub use position::{Position, StartPosition};

mod targets;
pub use targets::Targets;

#[derive(Clone, Copy, Component, Debug)]
pub struct King;

#[derive(Clone, Bundle)]
pub struct PieceBundle {
    pub behavior: Behavior,
    pub position: Position,
    pub start_position: StartPosition,
    pub team: Team,
    pub targets: Targets,
}

impl PieceBundle {
    pub fn new(behavior: Behavior, team: Team, start_position: StartPosition) -> Self {
        PieceBundle {
            behavior,
            position: start_position.to_position(&team),
            start_position,
            team,
            targets: Targets::default(),
        }
    }
}

#[derive(Clone, Bundle)]
pub struct PromotableBundle {
    pub piece: PieceBundle,
    pub promotable: Promotable,
}

impl PieceBundle {
    pub fn promotable(self, promotable: Promotable) -> PromotableBundle {
        PromotableBundle {
            piece: self,
            promotable,
        }
    }
}
