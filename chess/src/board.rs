use bevy::prelude::{Component, Deref, Entity, Reflect, ReflectComponent};

use fairy_gameboard::GameBoard;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[derive(Component, Deref, Reflect)]
#[reflect(Component)]
pub struct ChessBoard<B: GameBoard>(B);

impl<B: GameBoard> ChessBoard<B> {
    fn new(board: B) -> Self {
        Self::new(board)
    }
}

#[derive(Clone, Copy, Component, Debug, Reflect)]
pub struct OnBoard(pub Entity);
