use bevy_ecs::prelude::Component;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::board::Rank;

use super::PieceDefinition;

// AKA "Promotion", but named Mutation in case of more general purposes
// TODO: split condition and required into separate component types and systems?
#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct Mutation {
    // the Rank required to reach promotion
    pub condition: MutationCondition,
    // whether mutation is forced or optional
    pub required: MutationRequired,
    // the mutation options to choose from
    #[cfg_attr(feature = "reflect", reflect(ignore))]
    pub to_piece: Vec<PieceDefinition>,
    // whether the upgraded piece is royal
    pub to_royal: bool,
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum MutationCondition {
    // rank is local to team orientation
    LocalRank(Rank),
    OnCapture,
    // TODO: Region
    // TODO: ?????
}

impl Default for MutationCondition {
    fn default() -> Self {
        MutationCondition::LocalRank(Rank::EIGHT)
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum MutationRequired {
    #[default]
    Yes,
    No,
}
