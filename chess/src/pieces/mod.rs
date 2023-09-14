use bevy::prelude::{Bundle, Commands, Reflect};

use crate::team::Team;

mod behavior;
pub use behavior::{Behavior, EnPassantBehavior, MimicBehavior, PatternBehavior, RelayBehavior};

mod mutation;
pub use mutation::{Mutation, MutationCondition};

mod orientation;
pub use orientation::Orientation;

mod pattern;
pub use pattern::{
    ABSymmetry, CaptureMode, CapturePattern, CaptureRules, Pattern, RSymmetry, ScanMode,
    ScanTarget, Scanner, Step, TargetKind,
};

mod position;
pub use position::Position;

mod royal;
pub use royal::Royal;

// TODO move Action
mod actions;
pub use actions::{Action, Actions};

#[derive(Clone, Debug, Default, Reflect)]
pub struct PieceDefinition<Extra: Default = ()> {
    pub behavior: PatternBehavior,
    pub extra: Extra,
    pub mutation: Option<Mutation<Extra>>,
    pub royal: Option<Royal>,
}

pub struct PieceSpecification<Extra: Default = ()> {
    pub piece: PieceDefinition<Extra>,
    pub team: Team,
    pub start_position: Position,
    pub orientation: Orientation,
}

#[derive(Bundle)]
pub struct PieceBundle<Extra: Bundle + Default = ()> {
    pub behavior: PatternBehavior,
    pub team: Team,
    pub start_position: Position,
    pub orientation: Orientation,
    pub extra: Extra,
    pub actions: Actions,
}

impl<Extra: Default + Bundle + Clone> PieceSpecification<Extra> {
    pub fn new(piece: PieceDefinition<Extra>, team: Team, start_position: Position) -> Self {
        Self {
            piece,
            team,
            start_position,
            orientation: team.orientation(),
        }
    }

    pub fn bundle(self) -> PieceBundle<Extra> {
        PieceBundle {
            behavior: self.piece.behavior,
            team: self.team,
            start_position: self.start_position,
            orientation: self.orientation,
            extra: self.piece.extra,
            actions: Actions::default(),
        }
    }

    pub fn spawn(self, commands: &mut Commands) {
        match (self.piece.royal.clone(), self.piece.mutation.clone()) {
            (Some(_), Some(mutation)) => {
                commands.spawn((self.bundle(), Royal, mutation));
            }
            (Some(_), None) => {
                commands.spawn((self.bundle(), Royal));
            }
            (None, Some(mutation)) => {
                commands.spawn((self.bundle(), mutation));
            }
            _ => {}
        };
    }
}
