use bevy::prelude::{info, Bundle, Commands, Reflect};

use crate::team::Team;

mod behavior;
pub use behavior::{Behavior, Pattern, PatternStep, SearchMode, TargetMode};

mod mutation;
pub use mutation::{Mutation, MutationCondition};

mod position;
pub use position::Position;

mod royal;
pub use royal::Royal;

mod targets;
pub use targets::Targets;

#[derive(Clone, Debug, Default, Reflect)]
pub struct PieceDefinition<Extra: Default = ()> {
    pub behavior: Behavior,
    pub team: Team,
    pub extra: Extra,
    pub mutation: Option<Mutation<Extra>>,
    pub royal: Option<Royal>,
}

impl<Extras: Default + Bundle + Clone> PieceDefinition<Extras> {
    pub fn spawn(self, commands: &mut Commands, start_position: Position) {
        info!("{:?}", start_position);
        let entity = commands
            .spawn((
                self.behavior,
                start_position,
                self.team,
                Targets::default(),
                self.extra.clone(),
            ))
            .id();
        if let Some(mutation) = self.mutation {
            commands.entity(entity).insert(mutation);
        }
        if let Some(royal) = self.royal {
            commands.entity(entity).insert(royal);
        }
    }
}
