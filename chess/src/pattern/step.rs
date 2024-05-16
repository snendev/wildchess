use serde::{Deserialize, Serialize};

#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Deserialize, Serialize)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub enum Step {
    OneDim(i16, RSymmetry),
    TwoDim(i16, i16, ABSymmetry),
}

impl Step {
    // constructors
    pub fn from_r(r: i16, symmetry: RSymmetry) -> Self {
        Self::OneDim(r, symmetry)
    }

    pub fn from_ab(a: i16, b: i16, symmetry: ABSymmetry) -> Self {
        assert!(
            a > b && a != 0 && b != 0,
            "Invalid step constructed: Steps assume that `|a|>|b|` and  `a,b != 0`. Provided: a={}, b={}",
            a,
            b
        );
        Self::TwoDim(a, b, symmetry)
    }

    // common simple constructions
    pub fn forward(r: i16) -> Self {
        Step::from_r(r, RSymmetry::FORWARD)
    }

    pub fn horizontal(r: i16) -> Self {
        Step::from_r(r, RSymmetry::horizontal())
    }

    pub fn vertical(r: i16) -> Self {
        Step::from_r(r, RSymmetry::vertical())
    }

    pub fn backward(r: i16) -> Self {
        Step::from_r(r, RSymmetry::BACKWARD)
    }

    pub fn orthogonal(r: i16) -> Self {
        Step::from_r(r, RSymmetry::orthogonal())
    }

    pub fn diagonal_forward(r: i16) -> Self {
        Step::from_r(r, RSymmetry::diagonal_forward())
    }

    pub fn diagonal_backward(r: i16) -> Self {
        Step::from_r(r, RSymmetry::diagonal_backward())
    }

    pub fn diagonal(r: i16) -> Self {
        Step::from_r(r, RSymmetry::diagonal())
    }

    pub fn radial(r: i16) -> Self {
        Step::from_r(r, RSymmetry::all())
    }

    pub fn leaper(a: i16, b: i16) -> Self {
        Step::TwoDim(a, b, ABSymmetry::all())
    }

    pub fn forward_leaper(a: i16, b: i16) -> Self {
        Step::TwoDim(
            a,
            b,
            ABSymmetry::FORWARD_FORWARD_LEFT | ABSymmetry::FORWARD_FORWARD_RIGHT,
        )
    }

    // TODO some simple matrix ops might reduce line count a lot here
    // maybe don't even need a different symmetry type, but it's useful
    // to classify separate sets of constants
    fn r_symmetry_steps(r: i16, symmetry: RSymmetry) -> Vec<(i16, i16)> {
        let mut steps = vec![];
        if symmetry.intersects(RSymmetry::RIGHT) {
            steps.push((r, 0));
        }
        if symmetry.intersects(RSymmetry::FORWARD_RIGHT) {
            steps.push((r, r));
        }
        if symmetry.intersects(RSymmetry::FORWARD) {
            steps.push((0, r));
        }
        if symmetry.intersects(RSymmetry::FORWARD_LEFT) {
            steps.push((-r, r));
        }
        if symmetry.intersects(RSymmetry::LEFT) {
            steps.push((-r, 0));
        }
        if symmetry.intersects(RSymmetry::BACKWARD_LEFT) {
            steps.push((-r, -r));
        }
        if symmetry.intersects(RSymmetry::BACKWARD) {
            steps.push((0, -r));
        }
        if symmetry.intersects(RSymmetry::BACKWARD_RIGHT) {
            steps.push((r, -r));
        }
        steps
    }

    // TODO some simple matrix ops might reduce line count a lot here
    fn ab_symmetry_steps(a: i16, b: i16, symmetry: ABSymmetry) -> Vec<(i16, i16)> {
        let mut steps = vec![];
        if symmetry.intersects(ABSymmetry::FORWARD_RIGHT_RIGHT) {
            steps.push((a, b));
        }
        if symmetry.intersects(ABSymmetry::FORWARD_FORWARD_RIGHT) {
            steps.push((b, a));
        }
        if symmetry.intersects(ABSymmetry::FORWARD_FORWARD_LEFT) {
            steps.push((-b, a));
        }
        if symmetry.intersects(ABSymmetry::FORWARD_LEFT_LEFT) {
            steps.push((-a, b));
        }
        if symmetry.intersects(ABSymmetry::BACKWARD_LEFT_LEFT) {
            steps.push((-a, -b));
        }
        if symmetry.intersects(ABSymmetry::BACKWARD_BACKWARD_LEFT) {
            steps.push((-b, -a));
        }
        if symmetry.intersects(ABSymmetry::BACKWARD_BACKWARD_RIGHT) {
            steps.push((b, -a));
        }
        if symmetry.intersects(ABSymmetry::BACKWARD_RIGHT_RIGHT) {
            steps.push((a, -b));
        }
        steps
    }

    pub fn movements(&self) -> Vec<(i16, i16)> {
        match self {
            Step::OneDim(r, symmetry) => Self::r_symmetry_steps(*r, *symmetry),
            Step::TwoDim(a, b, symmetry) => Self::ab_symmetry_steps(*a, *b, *symmetry),
        }
    }
}

impl Default for Step {
    fn default() -> Self {
        Step::OneDim(1, RSymmetry::default())
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[derive(Deserialize, Serialize)]
    #[cfg_attr(feature = "reflect", derive(Reflect))]
    #[cfg_attr(feature = "reflect", reflect_value)]
    pub struct RSymmetry: u8 {
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

impl RSymmetry {
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

impl Default for RSymmetry {
    fn default() -> Self {
        Self::ALL
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[derive(Deserialize, Serialize)]
    #[cfg_attr(feature = "reflect", derive(Reflect))]
    #[cfg_attr(feature = "reflect", reflect_value)]
    pub struct ABSymmetry: u8 {
        const FORWARD_RIGHT_RIGHT = 0b00000001;
        const FORWARD_FORWARD_RIGHT = 0b00000010;
        const FORWARD_FORWARD_LEFT = 0b00000100;
        const FORWARD_LEFT_LEFT = 0b00001000;
        const BACKWARD_LEFT_LEFT = 0b00010000;
        const BACKWARD_BACKWARD_LEFT = 0b00100000;
        const BACKWARD_BACKWARD_RIGHT = 0b01000000;
        const BACKWARD_RIGHT_RIGHT = 0b10000000;

        const ALL = 0b11111111;
    }
}

impl ABSymmetry {
    pub fn narrow_forward() -> Self {
        Self::FORWARD_FORWARD_LEFT | Self::FORWARD_FORWARD_RIGHT
    }

    pub fn wide_forward() -> Self {
        Self::FORWARD_RIGHT_RIGHT | Self::FORWARD_LEFT_LEFT
    }

    pub fn all_forward() -> Self {
        Self::narrow_forward() | Self::wide_forward()
    }

    pub fn narrow_backward() -> Self {
        Self::BACKWARD_BACKWARD_LEFT | Self::BACKWARD_BACKWARD_RIGHT
    }

    pub fn wide_backward() -> Self {
        Self::BACKWARD_RIGHT_RIGHT | Self::BACKWARD_LEFT_LEFT
    }

    pub fn all_backward() -> Self {
        Self::narrow_backward() | Self::wide_backward()
    }
}

impl Default for ABSymmetry {
    fn default() -> Self {
        Self::ALL
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
