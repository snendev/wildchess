use bevy::prelude::{Bundle, Commands, Entity, Reflect};

use crate::{behavior::PieceBehaviors, team::Team};

mod mutation;
pub use mutation::{Mutation, MutationCondition, MutationRequired};

mod orientation;
pub use orientation::Orientation;

// TODO: move Pattern code
mod pattern;
pub use pattern::{
    ABSymmetry, CaptureMode, CapturePattern, CaptureRules, Constraints, ForbiddenTargetConstraint,
    FromRankConstraint, Pattern, RSymmetry, ScanMode, ScanTarget, Scanner, Step, TargetKind,
};

mod position;
pub use position::Position;

mod royal;
pub use royal::Royal;

// TODO move Action code
mod actions;
pub use actions::{Action, Actions};

#[derive(Clone, Debug, Default, Bundle, Reflect)]
pub struct PieceState {
    pub team: Team,
    pub position: Position,
    pub orientation: Orientation,
}

impl PieceState {
    pub fn new(start_position: Position, team: Team) -> Self {
        PieceState {
            team,
            position: start_position,
            orientation: team.orientation(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Bundle, Reflect)]
pub struct PieceBundle {
    pub start_state: PieceState,
    pub actions: Actions,
}

impl PieceBundle {
    pub fn new(start_position: Position, team: Team) -> Self {
        PieceBundle {
            start_state: PieceState::new(start_position, team),
            ..Default::default()
        }
    }

    pub fn from_state(state: PieceState) -> Self {
        PieceBundle {
            start_state: state,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Reflect)]
pub struct PieceDefinition {
    pub behaviors: PieceBehaviors,
    pub mutation: Option<Mutation>,
    pub royal: Option<Royal>,
}

impl PieceDefinition {
    pub fn new(behaviors: PieceBehaviors) -> Self {
        PieceDefinition {
            behaviors,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Default, Reflect)]
pub struct PieceSpecification {
    pub piece: PieceDefinition,
    pub start_state: PieceState,
}

impl PieceSpecification {
    pub fn new(piece: PieceDefinition, team: Team, start_position: Position) -> Self {
        Self {
            piece,
            start_state: PieceState::new(start_position, team),
        }
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn(PieceBundle::from_state(self.start_state))
            .id();

        if self.piece.royal.is_some() {
            commands.entity(entity).insert(Royal);
        }
        if let Some(mutation) = self.piece.mutation {
            commands.entity(entity).insert(mutation);
        }
        if let Some(behavior) = self.piece.behaviors.en_passant {
            commands.entity(entity).insert(behavior);
        }
        if let Some(behavior) = self.piece.behaviors.mimic {
            commands.entity(entity).insert(behavior);
        }
        if let Some(behavior) = self.piece.behaviors.pattern {
            commands.entity(entity).insert(behavior.clone());
        }
        if let Some(behavior) = self.piece.behaviors.relay {
            commands.entity(entity).insert(behavior);
        }

        entity
    }
}
