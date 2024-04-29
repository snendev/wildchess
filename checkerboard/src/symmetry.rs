use enumflags2::{bitflags, BitFlags};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

// Each square's potential steps, given some vector (a: i16, b: i16),
// can be transformed using the Rotational symmetry of an octogon.
// when a!=b, a,b!=0 this creates combinations (a, b), (b, a), (-a, b), ..., (-b, -a)
// whereas when a=b, a=0, or b=0, this creates combinations
// (0, r), (r, r), (0, r), (r, -r), (0, -r), (-r, -r), ...
// (i.e. theta=0,pi/4,pi/2,...,7pi/4 radians, from y=0, r as the nonzero element)
// This struct is a representation of that symmetry, represented by cardinal and
// ordinal directions to make the reasoning a little easier
#[bitflags(default = Forward)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum GridSymmetry {
    // these are the rotations that a vector could be symmetric over
    // `Step`s will be executed for all rotations specified.
    #[default]
    Forward = 1 << 0,
    ForwardRight = 1 << 1,
    Right = 1 << 2,
    BackwardRight = 1 << 3,
    Backward = 1 << 4,
    BackwardLeft = 1 << 5,
    Left = 1 << 6,
    ForwardLeft = 1 << 7,
}

use GridSymmetry::*;

use crate::Square;

// N.B. using this in the above definition would create a cycle, but
impl GridSymmetry {
    fn bit_offset(self) -> u32 {
        match self {
            Forward => 0,
            ForwardRight => 1,
            Right => 2,
            BackwardRight => 3,
            Backward => 4,
            BackwardLeft => 5,
            Left => 6,
            ForwardLeft => 7,
        }
    }
}

impl std::ops::Mul<BitFlags<Self>> for GridSymmetry {
    type Output = BitFlags<Self>;

    fn mul(self, rhs: BitFlags<Self>) -> BitFlags<Self> {
        BitFlags::from_bits(rhs.bits().rotate_left(self.bit_offset()))
            .expect("Failed to rotate bitflags")
    }
}

impl std::ops::Mul<Square> for GridSymmetry {
    type Output = Square;

    fn mul(self, rhs: Square) -> Self::Output {
        let a = *rhs.rank();
        let b = *rhs.file();
        let ra = if a == 0 { b } else { a };
        let rb = if b == 0 { a } else { b };
        match self {
            Forward => Square::from_values(b, a),
            ForwardRight => Square::from_values(ra, rb),
            Right => Square::from_values(a, -b),
            BackwardRight => Square::from_values(rb, -ra),
            Backward => Square::from_values(-b, -a),
            BackwardLeft => Square::from_values(-ra, -rb),
            Left => Square::from_values(-a, b),
            ForwardLeft => Square::from_values(-rb, ra),
        }
    }
}

impl GridSymmetry {
    // RSymmetry
    pub fn all_forward() -> BitFlags<Self> {
        Self::ForwardRight | Self::Forward | Self::ForwardLeft
    }

    pub fn all_right() -> BitFlags<Self> {
        Self::Right | Self::ForwardRight | Self::BackwardRight
    }

    pub fn all_backward() -> BitFlags<Self> {
        Self::BackwardLeft | Self::Backward | Self::BackwardRight
    }

    pub fn all_left() -> BitFlags<Self> {
        Self::ForwardLeft | Self::Left | Self::BackwardLeft
    }

    pub fn vertical() -> BitFlags<Self> {
        Self::Forward | Self::Backward
    }

    pub fn horizontal() -> BitFlags<Self> {
        Self::Left | Self::Right
    }

    pub fn sideways() -> BitFlags<Self> {
        Self::horizontal()
    }

    pub fn orthogonal() -> BitFlags<Self> {
        Self::Forward | Self::Left | Self::Right | Self::Backward
    }

    pub fn diagonal_forward() -> BitFlags<Self> {
        Self::ForwardRight | Self::ForwardLeft
    }

    pub fn diagonal_backward() -> BitFlags<Self> {
        Self::BackwardLeft | Self::BackwardRight
    }

    pub fn diagonal() -> BitFlags<Self> {
        Self::diagonal_forward() | Self::diagonal_backward()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ensure bit offsets work as expected
    #[test]
    fn test_bit_offset() {
        assert_eq!(Forward.bit_offset(), 0);
        assert_eq!(ForwardRight.bit_offset(), 1);
        assert_eq!(Right.bit_offset(), 2);
        assert_eq!(BackwardRight.bit_offset(), 3);
        assert_eq!(Backward.bit_offset(), 4);
        assert_eq!(BackwardLeft.bit_offset(), 5);
        assert_eq!(Left.bit_offset(), 6);
        assert_eq!(ForwardLeft.bit_offset(), 7);
    }

    // verify that the identity is valid
    #[test]
    fn test_identity_orientation() {
        let orientation = Forward;

        let all_forward = GridSymmetry::all_forward();
        assert_eq!(orientation * all_forward, all_forward);

        let all_right = GridSymmetry::all_right();
        assert_eq!(orientation * all_right, all_right);

        let all_backward = GridSymmetry::all_backward();
        assert_eq!(orientation * all_backward, all_backward);
    }

    // test a smattering of cases

    #[test]
    fn test_forward_right_orientation() {
        let orientation = ForwardRight;

        let all_forward = GridSymmetry::all_forward();
        assert_eq!(orientation * all_forward, Forward | ForwardRight | Right);

        let all_right = GridSymmetry::all_right();
        assert_eq!(orientation * all_right, Right | BackwardRight | Backward);

        let all_backward = GridSymmetry::all_backward();
        assert_eq!(orientation * all_backward, Backward | BackwardLeft | Left);

        let all_left = GridSymmetry::all_left();
        assert_eq!(orientation * all_left, Left | ForwardLeft | Forward);
    }

    #[test]
    fn test_right_orientation() {
        let orientation = Right;

        let all_forward = GridSymmetry::all_forward();
        let all_right = GridSymmetry::all_right();
        let all_backward = GridSymmetry::all_backward();
        let all_left = GridSymmetry::all_left();

        assert_eq!(orientation * all_forward, all_right);
        assert_eq!(orientation * all_right, all_backward);
        assert_eq!(orientation * all_backward, all_left);
        assert_eq!(orientation * all_left, all_forward);
    }

    #[test]
    fn test_backward_orientation() {
        let orientation = Backward;

        let all_forward = GridSymmetry::all_forward();
        let all_right = GridSymmetry::all_right();
        let all_backward = GridSymmetry::all_backward();
        let all_left = GridSymmetry::all_left();

        assert_eq!(orientation * all_forward, all_backward);
        assert_eq!(orientation * all_backward, all_forward);
        assert_eq!(orientation * all_right, all_left);
        assert_eq!(orientation * all_left, all_right);
    }

    #[test]
    fn test_back_left_orientation() {
        let orientation = BackwardLeft;

        assert_eq!(orientation * BitFlags::from_flag(Forward), BackwardLeft);
        assert_eq!(orientation * BitFlags::from_flag(Right), ForwardLeft);
        assert_eq!(orientation * BitFlags::from_flag(BackwardLeft), Right);
        assert_eq!(orientation * BitFlags::from_flag(Backward), ForwardRight);
    }
}
