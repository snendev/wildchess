use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Mul};

use enumflags2::{BitFlag, BitFlags};

/// Used to define how vectors are laid out in board space.
///
/// Across board types, vectors must be able to be added and scaled.
/// They are also used in equality comparisons and as hash keys.
pub trait BoardVector:
    Add<Output = Self>
    + Mul<i16, Output = Self>
    + Send
    + Sync
    + Clone
    + Copy
    + Debug
    + Default
    + PartialEq
    + Eq
    + Hash
{
    /// Defines the possible reflections and rotations that could operate on a BoardVector.
    ///
    /// This type can be used to define the "orientation" upon which a particular `BoardVector` should be applied.
    /// For example, on a typical chessboard, pieces on opposing teams have "backward" orientation relative to each other.
    /// They must implement `Mul<BitFlags<Self::Symmetry>>` so that these symmetries can be composed accordingly:
    /// specifically, an arbitary union of orientations can be "rotated" by another orientation
    type Symmetry: BitFlag
        + Mul<BitFlags<Self::Symmetry>, Output = BitFlags<Self::Symmetry>>
        + Mul<Self, Output = Self>
        + Send
        + Sync
        + Clone
        + Copy
        + Debug
        + Default
        + PartialEq;

    // TODO, maybe: type Notation;

    fn collect_symmetries(&self, symmetries: BitFlags<Self::Symmetry>) -> Vec<Self>;
}
