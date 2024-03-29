use bevy::prelude::{Component, Entity, Reflect};

use board::*;

mod square;
pub use square::{File, Rank, Square};

#[derive(Clone, Copy, Component, Debug, Reflect)]
pub struct OnBoard(pub Entity);

#[derive(Clone, Copy, Component, Debug, Default, Reflect)]
pub struct CheckerBoard {
    pub size: Square,
}

impl CheckerBoard {
    pub fn chess_board() -> Self {
        Self {
            size: Square::new(File::H, Rank::EIGHT),
        }
    }

    pub fn shogi_board() -> Self {
        Self {
            size: Square::new(File(8), Rank(8)),
        }
    }
}

impl GameBoard for CheckerBoard {
    type Position = Square;
    type Axes = Grid;

    fn is_in_bounds(&self, position: Self::Position) -> bool {
        
    }
}

// Each square's potential steps, given some vector (a: i16, b: i16),
// can be transformed using the Rotational symmetry of an octogon.
// when a!=b, a,b!=0 this creates combinations (a, b), (b, a), (-a, b), ..., (-b, -a)
// whereas when a=b, a=0, or b=0, this creates combinations
// (0, r), (r, r), (0, r), (r, -r), (0, -r), (-r, -r), ...
// (i.e. theta=0,pi/4,pi/2,...,7pi/4 radians, from y=0, r as the nonzero element)
// This struct is a representation of that symmetry, represented by cardinal and
// ordinal directions to make the reasoning a little easier
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    #[derive(Reflect)]
    #[reflect_value]
    pub struct Grid: u8 {
        // these are the rotations that a vector could be symmetric over
        // `Step`s will be executed for all rotations specified.
        const RIGHT = 0b00000001;
        const FORWARD_RIGHT = 0b00000010;
        const FORWARD  = 0b00000100;
        const FORWARD_LEFT = 0b00001000;
        const LEFT = 0b00010000;
        const BACKWARD_LEFT = 0b00100000;
        const BACKWARD = 0b01000000;
        const BACKWARD_RIGHT = 0b10000000;

        const ALL = 0b11111111;
    }
}
impl Grid {
    // RSymmetry
    pub fn all_forward() -> Self {
        Self::FORWARD_RIGHT | Self::FORWARD | Self::FORWARD_LEFT
    }

    pub fn all_right() -> Self {
        Self::RIGHT | Self::FORWARD_RIGHT | Self::BACKWARD_RIGHT
    }

    pub fn all_backward() -> Self {
        Self::BACKWARD_LEFT | Self::BACKWARD | Self::BACKWARD_RIGHT
    }

    pub fn all_left() -> Self {
        Self::FORWARD_LEFT | Self::LEFT | Self::BACKWARD_LEFT
    }

    pub fn vertical() -> Self {
        Self::FORWARD | Self::BACKWARD
    }

    pub fn horizontal() -> Self {
        Self::LEFT | Self::RIGHT
    }

    pub fn sideways() -> Self {
        Self::horizontal()
    }

    pub fn orthogonal() -> Self {
        Self::FORWARD | Self::LEFT | Self::RIGHT | Self::BACKWARD
    }

    pub fn diagonal_forward() -> Self {
        Self::FORWARD_RIGHT | Self::FORWARD_LEFT
    }

    pub fn diagonal_backward() -> Self {
        Self::BACKWARD_LEFT | Self::BACKWARD_RIGHT
    }

    pub fn diagonal() -> Self {
        Self::diagonal_forward() | Self::diagonal_backward()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rook_stepper() {
        let rook = Step::orthogonal(1);
        let mut results = rook.movements();
        results.sort();

        let mut correct = vec![
            // up
            (0, 1),
            // down
            (0, -1),
            // right
            (1, 0),
            // left
            (-1, 0),
        ];
        correct.sort();

        assert_eq!(
            results,
            correct,
            "Scanner yielded squares: {:?}",
            results
                .iter()
                .map(|point| format!("Vec({}, {})", point.0, point.1))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_bishop_stepper() {
        let bishop = Step::diagonal(1);
        let mut results = bishop.movements();
        results.sort();

        let mut correct = vec![
            // up left
            (-1, 1),
            // up right
            (1, 1),
            // down left
            (-1, -1),
            // down right
            (1, -1),
        ];
        correct.sort();

        assert_eq!(
            results,
            correct,
            "Scanner yielded squares: {:?}",
            results
                .iter()
                .map(|point| format!("Vec({}, {})", point.0, point.1))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_knight_stepper() {
        let knight = Step::leaper(2, 1);
        let mut results = knight.movements();
        results.sort();

        let mut correct = vec![
            // up right
            (2, 1),
            (1, 2),
            // up left
            (-2, 1),
            (-1, 2),
            // down right
            (2, -1),
            (1, -2),
            // down left
            (-2, -1),
            (-1, -2),
        ];
        correct.sort();

        assert_eq!(
            results,
            correct,
            "Scanner yielded squares: {:?}",
            results
                .iter()
                .map(|point| format!("Vec({}, {})", point.0, point.1))
                .collect::<Vec<_>>()
        );
    }
}
