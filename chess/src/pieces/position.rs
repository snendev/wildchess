use bevy_derive::Deref;
use bevy_ecs::prelude::Component;
#[cfg(feature = "reflect")]
use bevy_ecs::prelude::ReflectComponent;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use fairy_gameboard::{BoardVector, GameBoard};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[derive(Component, Deref)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct Position<B: GameBoard>(B::Vector);

impl<B: GameBoard> Position<B> {
    pub fn new(position: B::Vector) -> Self {
        Self::new(position)
    }

    pub fn set(&mut self, position: B::Vector) {
        self.0 = position;
    }
}

// Once all Royal pieces are captured, a player loses the game.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[derive(Component, Deref)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct Orientation<B: GameBoard>(<B::Vector as BoardVector>::Symmetry);
