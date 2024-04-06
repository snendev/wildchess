#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{BoardVector, GameBoard};

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Movement<B: GameBoard> {
    from: B::Vector,
    to: B::Vector,
    orientation: <B::Vector as BoardVector>::Symmetry,
}

impl<B: GameBoard> Movement<B> {
    pub fn new(
        from: B::Vector,
        to: B::Vector,
        orientation: <B::Vector as BoardVector>::Symmetry,
    ) -> Self {
        Self {
            from,
            to,
            orientation,
        }
    }

    pub fn from(&self) -> B::Vector {
        self.from
    }

    pub fn to(&self) -> B::Vector {
        self.to
    }

    pub fn orientation(&self) -> <B::Vector as BoardVector>::Symmetry {
        self.orientation
    }
}
