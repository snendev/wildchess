use bevy::{
    ecs::system::EntityCommands,
    prelude::{Bundle, Commands, Name, Reflect},
};

use crate::{actions::Actions, behavior::PieceBehaviors, team::Team};

mod identity;
pub use identity::PieceIdentity;

mod mutation;
pub use mutation::{Mutation, MutationCondition, MutationRequired};

mod orientation;
pub use orientation::Orientation;

mod position;
pub use position::Position;

mod royal;
pub use royal::Royal;

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
    pub identity: PieceIdentity,
    pub mutation: Option<Mutation>,
    pub royal: Option<Royal>,
}

impl PieceDefinition {
    pub fn new(behaviors: PieceBehaviors, identity: PieceIdentity) -> Self {
        PieceDefinition {
            behaviors,
            identity,
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

    pub fn spawn<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
    ) -> EntityCommands<'w, 's, 'a> {
        let name = Name::new(format!(
            "{:?} {}-{:?}",
            self.start_state.team, self.start_state.position.0, self.piece.identity,
        ));

        let mut builder = commands.spawn((
            PieceBundle::from_state(self.start_state),
            self.piece.identity,
            name,
        ));

        if self.piece.royal.is_some() {
            builder.insert(Royal);
        }
        if let Some(mutation) = self.piece.mutation {
            builder.insert(mutation);
        }
        if let Some(behavior) = self.piece.behaviors.en_passant {
            builder.insert(behavior);
        }
        if let Some(behavior) = self.piece.behaviors.mimic {
            builder.insert(behavior);
        }
        if let Some(behavior) = self.piece.behaviors.pattern {
            builder.insert(behavior.clone());
        }
        if let Some(behavior) = self.piece.behaviors.relay {
            builder.insert(behavior);
        }

        builder
    }
}
