use bevy_derive::Deref;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::prelude::{Component, Entity};
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use fairy_gameboard::GameBoard;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[derive(Component, Deref)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct ChessBoard<B: GameBoard>(B);

impl<B: GameBoard> ChessBoard<B> {
    fn new(board: B) -> Self {
        Self::new(board)
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct OnBoard(pub Entity);
