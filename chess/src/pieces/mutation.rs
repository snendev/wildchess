use bevy::prelude::{Component, Reflect, ReflectComponent};

use crate::square::Rank;

use super::PieceDefinition;

#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Mutation<Extra: Default = ()> {
    // the Rank required to reach promotion
    pub condition: MutationCondition,
    // the upgraded Behaviors to choose from
    pub options: Vec<PieceDefinition<Extra>>,
}

#[derive(Clone, Debug, Reflect)]
pub enum MutationCondition {
    Rank(Rank),
    // TODO: Region
    // TODO: ?????
}

impl Default for MutationCondition {
    fn default() -> Self {
        MutationCondition::Rank(Rank::default())
    }
}
