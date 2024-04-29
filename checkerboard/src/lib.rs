use enumflags2::BitFlags;

use bevy_ecs::prelude::Component;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

use fairy_gameboard::*;

mod square;
pub use square::*;

mod symmetry;
pub use symmetry::*;

#[derive(Clone, Debug, Default)]
#[derive(Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
pub struct CheckerBoard {
    // TODO: forbid overlap?
    // probably implies developer error
    regions: Vec<(Square, Square)>,
}

impl CheckerBoard {
    pub fn chess_board() -> Self {
        Self {
            regions: vec![(Square::ZERO, Square::new(File::H, Rank::EIGHT))],
        }
    }

    pub fn shogi_board() -> Self {
        Self {
            regions: vec![(Square::ZERO, Square::from_values(8, 8))],
        }
    }
}

impl GameBoard for CheckerBoard {
    type Vector = Square;

    fn is_in_bounds(&self, position: Self::Vector) -> bool {
        self.regions.iter().any(|(min, max)| {
            position.file() >= min.file()
                && position.file() <= max.file()
                && position.rank() >= min.rank()
                && position.rank() <= max.rank()
        })
    }
}

impl BoardVector for Square {
    type Symmetry = GridSymmetry;

    fn collect_symmetries(&self, symmetries: BitFlags<Self::Symmetry>) -> Vec<Self> {
        use GridSymmetry::*;
        let mut steps = vec![];
        for orientation in [
            Forward,
            ForwardRight,
            Right,
            BackwardRight,
            Backward,
            BackwardLeft,
            Left,
            ForwardLeft,
        ] {
            if symmetries.contains(orientation) {
                steps.push(orientation * *self);
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use enumflags2::BitFlag;

    use super::*;

    #[test]
    fn test_rook_movement() {
        let forward_vector = Square::from_values(0, 1);
        let rook_symmetries = GridSymmetry::orthogonal();
        let rook_movements = forward_vector.collect_symmetries(rook_symmetries);

        let correct = vec![
            // forward
            Square::from_values(0, 1),
            // right
            Square::from_values(1, 0),
            // backward
            Square::from_values(0, -1),
            // left
            Square::from_values(-1, 0),
        ];

        assert_eq!(rook_movements, correct);
    }

    #[test]
    fn test_bishop_movement() {
        let forward_vector = Square::from_values(0, 1);
        let bishop_symmetries = GridSymmetry::diagonal();
        let bishop_movements = forward_vector.collect_symmetries(bishop_symmetries);

        let correct = vec![
            // forward right
            Square::from_values(1, 1),
            // backward right
            Square::from_values(1, -1),
            // backward left
            Square::from_values(-1, -1),
            // forward left
            Square::from_values(-1, 1),
        ];

        assert_eq!(bishop_movements, correct);
    }

    #[test]
    fn test_queen_movement() {
        let forward_vector = Square::from_values(0, 1);
        let queen_symmetries = GridSymmetry::all();
        let queen_movements = forward_vector.collect_symmetries(queen_symmetries);

        let correct = vec![
            // forward
            Square::from_values(0, 1),
            // forward right
            Square::from_values(1, 1),
            // right
            Square::from_values(1, 0),
            // backward right
            Square::from_values(1, -1),
            // backward
            Square::from_values(0, -1),
            // backward left
            Square::from_values(-1, -1),
            // left
            Square::from_values(-1, 0),
            // forward left
            Square::from_values(-1, 1),
        ];

        assert_eq!(queen_movements, correct);
    }

    #[test]
    fn test_knight_stepper() {
        let knight_vector = Square::from_values(1, 2);
        let knight_symmetries = GridSymmetry::all();
        let knight_movements = knight_vector.collect_symmetries(knight_symmetries);

        let correct = vec![
            // up up right
            Square::from_values(1, 2),
            // up right right
            Square::from_values(2, 1),
            // down right right
            Square::from_values(2, -1),
            // down down right
            Square::from_values(1, -2),
            // down down left
            Square::from_values(-1, -2),
            // down left left
            Square::from_values(-2, -1),
            // up left left
            Square::from_values(-2, 1),
            // up up left
            Square::from_values(-1, 2),
        ];

        assert_eq!(knight_movements, correct);
    }

    #[test]
    fn test_bounds() {
        let board = CheckerBoard::chess_board();
        assert!(board.is_in_bounds(Square::from_values(0, 0)));
        assert!(board.is_in_bounds(Square::from_values(1, 1)));
        assert!(board.is_in_bounds(Square::from_values(7, 7)));
        assert!(!board.is_in_bounds(Square::from_values(-1, 0)));
        assert!(!board.is_in_bounds(Square::from_values(0, -1)));
        assert!(!board.is_in_bounds(Square::from_values(-7, -7)));
    }

    #[test]
    fn test_irregular_bounds() {
        let board = CheckerBoard {
            regions: vec![
                (Square::ZERO, Square::from_values(4, 4)),
                (Square::from_values(-5, -5), Square::from_values(-1, -1)),
            ],
        };

        assert!(board.is_in_bounds(Square::from_values(0, 0)));
        assert!(board.is_in_bounds(Square::from_values(3, 4)));
        assert!(!board.is_in_bounds(Square::from_values(6, 6)));
        assert!(!board.is_in_bounds(Square::from_values(-1, 0)));
        assert!(board.is_in_bounds(Square::from_values(-1, -1)));
        assert!(board.is_in_bounds(Square::from_values(-5, -5)));
        assert!(!board.is_in_bounds(Square::from_values(-8, -8)));
    }
}
