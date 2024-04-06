use bevy::prelude::{Component, Deref, Reflect, ReflectComponent};
use fairy_gameboard::{BoardVector, GameBoard};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[derive(Component, Deref, Reflect)]
#[reflect(Component)]
pub struct Position<B: GameBoard>(B::Vector);

impl <B: GameBoard> Position<B> {
    fn new(position: B::Vector) -> Self {
        Self::new(position)
    }
}

// Once all Royal pieces are captured, a player loses the game.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[derive(Component, Deref, Reflect)]
#[reflect(Component)]
pub struct Orientation<B: GameBoard>(<B::Vector as BoardVector>::Symmetry);
