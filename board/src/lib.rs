use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Mul};

mod actions;
pub use actions::*;

mod step;
pub use step::*;

pub trait BoardVector:
    Clone
    + Copy
    + Debug
    + Default
    + PartialEq
    + Eq
    + Hash
    + Add<Output = Self>
    + Mul<u16, Output = Self>
{
    type Symmetry;
    type Team;

    // ??: type Notation;

    fn reflect_symmetries(&self, symmetry: Self::Symmetry) -> impl Iterator<Item = Self>;
}

pub trait GameBoard: Default + Sized {
    // Position defines some vector that can be added and scaled
    type Position: BoardVector;
    // Axes are the symmetries along which a step could be taken
    type Axes: Clone + Copy + Debug + Default + Into<Self::Position>;

    fn is_in_bounds(&self, position: Self::Position) -> bool;

    fn scan(&self, origin: Self::Position, step: BoardStep<Self>) -> BoardIterator<'_, Self> {
        BoardIterator {
            board: self,
            current_position: origin,
            step,
        }
    }
}

pub struct BoardIterator<'a, B: GameBoard> {
    board: &'a B,
    current_position: B::Position,
    step: BoardStep<B>,
}

impl<'a, B: GameBoard> Iterator for BoardIterator<'a, B> {
    // TODO maybe iter returns (Square, DoesCollide)
    type Item = B::Position;

    fn next(&mut self) -> Option<Self::Item> {
        let next_position = self.step.take_step(self.current_position);
        if self.board.is_in_bounds(next_position) {
            self.current_position = next_position;
            Some(self.current_position)
        } else {
            None
        }
    }
}
