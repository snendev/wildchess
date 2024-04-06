use enumflags2::BitFlags;

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use crate::{BoardVector, GameBoard};

// The calculation type for board searches
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct Pattern<B: GameBoard> {
    // the vector used to execute each step in searching the board
    pub step: B::Vector,
    // the symmetries along which the step can be executed
    pub symmetries: BitFlags<<B::Vector as BoardVector>::Symmetry>,
}

impl<B: GameBoard> Pattern<B> {
    pub fn new(
        step: B::Vector,
        symmetries: BitFlags<<B::Vector as BoardVector>::Symmetry>,
    ) -> Self {
        Pattern { step, symmetries }
    }

    pub fn collect_steps(
        &self,
        orientation: <B::Vector as BoardVector>::Symmetry,
    ) -> Vec<B::Vector> {
        (orientation * self.step).collect_symmetries(self.symmetries)
    }
}

// TODO: rather that constructing an iterator in a dynamic-programming-like fashion, consider an API where users query the pattern on a sample of positions.
// Then it could return an iterator of "paths" which could then be tested for collisions, or accept the list of user-defined collider positions up-front.
//
// How to represent this?
// We don't really want "step & orientations", we want something more like the symmetries established by group theory.
// A symmetry creates an equivalence class of translations, rotations, and reflections, and we iterate by expanding combinations of those symmetries.
// Perhaps we could express this using some rotation matrix format, but it would be ideal to avoid floating point operations since board positions should(?) always be whole units.
//
// Questions:
// How do the performance characteristics of these modes differ in reasonable contexts (views over infinite boards, simple boards, rpg boards)?
// Is the user experience of visiting each "path" bad? (perhaps users could "query" the paths for colliders etc as well)
// Is this version meaningfully helpful for purposes other than infinite boards?
//
// pub struct QueryablePattern<B: GameBoard> {
//     pub step: B::Vector,
// }
//
// impl <B: GameBoard>QueryablePattern<B> {
//     pub fn sample_positions(_positions: Vec<B::Vector>) -> impl Iterator<Item = impl Iterator<Item = B::Vector>> {
//         std::iter::once(std::iter::empty())
//     }
// }
