use bevy::prelude::{Component, Reflect, ReflectComponent};
use fairy_gameboard::GameBoard;

use super::PieceDefinition;

// AKA "Promotion", but named Mutation in case of more general purposes
// TODO: split condition and required into separate component types and systems?
#[derive(Clone, Debug, Default)]
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Mutation<B: GameBoard> {
    // the Rank required to reach promotion
    pub condition: MutationCondition<B>,
    // whether mutation is forced or optional
    pub required: MutationRequired,
    // the mutation options to choose from
    #[reflect(ignore)]
    pub to_piece: Vec<PieceDefinition>,
    // whether the upgraded piece is royal
    pub to_royal: bool,
}

#[derive(Clone, Debug)]
pub enum MutationCondition<B: GameBoard> {
    Region(Vec<B::Vector>),
    OnCapture,
    // TODO: ?????
}

#[derive(Clone, Debug, Default, Reflect)]
pub enum MutationRequired {
    #[default]
    Yes,
    No,
}
