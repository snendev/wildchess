use crate::GameBoard;

#[derive(Clone, Debug, Default)]
pub struct BoardStep<B: GameBoard> {
    substeps: Vec<(u16, B::Axes)>   
}

impl <B: GameBoard>BoardStep<B> {
    pub fn go(axes: B::Axes, distance: u16) -> Self {
        Self {
            substeps: vec![(distance, axes)]
        }
    }

    pub fn followed_by(mut self, distance: u16, axes: B::Axes) -> Self {
        self.substeps.push((distance, axes));
        self
    }

    pub(crate) fn take_step(&self, position: B::Position) -> B::Position {
        self.substeps.iter().fold(position, |sum, (distance, axes)| sum + (*axes).into() * *distance)
    }
}

#[cfg(test)]
mod tests {
//     use super::*;

//     #[test]
//     fn test_rook_stepper() {
//         let rook = Step::orthogonal(1);
//         let mut results = rook.movements();
//         results.sort();

//         let mut correct = vec![
//             // up
//             (0, 1),
//             // down
//             (0, -1),
//             // right
//             (1, 0),
//             // left
//             (-1, 0),
//         ];
//         correct.sort();

//         assert_eq!(
//             results,
//             correct,
//             "Scanner yielded squares: {:?}",
//             results
//                 .iter()
//                 .map(|point| format!("Vec({}, {})", point.0, point.1))
//                 .collect::<Vec<_>>()
//         );
//     }

//     #[test]
//     fn test_bishop_stepper() {
//         let bishop = Step::diagonal(1);
//         let mut results = bishop.movements();
//         results.sort();

//         let mut correct = vec![
//             // up left
//             (-1, 1),
//             // up right
//             (1, 1),
//             // down left
//             (-1, -1),
//             // down right
//             (1, -1),
//         ];
//         correct.sort();

//         assert_eq!(
//             results,
//             correct,
//             "Scanner yielded squares: {:?}",
//             results
//                 .iter()
//                 .map(|point| format!("Vec({}, {})", point.0, point.1))
//                 .collect::<Vec<_>>()
//         );
//     }

//     #[test]
//     fn test_knight_stepper() {
//         let knight = Step::leaper(2, 1);
//         let mut results = knight.movements();
//         results.sort();

//         let mut correct = vec![
//             // up right
//             (2, 1),
//             (1, 2),
//             // up left
//             (-2, 1),
//             (-1, 2),
//             // down right
//             (2, -1),
//             (1, -2),
//             // down left
//             (-2, -1),
//             (-1, -2),
//         ];
//         correct.sort();

//         assert_eq!(
//             results,
//             correct,
//             "Scanner yielded squares: {:?}",
//             results
//                 .iter()
//                 .map(|point| format!("Vec({}, {})", point.0, point.1))
//                 .collect::<Vec<_>>()
//         );
//     }
}
