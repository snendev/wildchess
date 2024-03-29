use bevy::{prelude::Reflect, utils::HashMap};

use crate::{
    board::{Board, Square},
    pieces::Orientation,
    team::Team,
};

use super::{Step, TargetKind};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub enum ScanMode {
    #[default]
    // Step until reaching a colliding piece
    // (Includes the colliding square to account for captures)
    Walk,
    // Step until at max range, ignoring colliding pieces
    Pierce,
    // Ignore any steps until reaching a colliding piece
    // Movement can occur starting on the colliding piece square
    // (Includes the colliding square to account for captures)
    Hop {
        max_steps_after_hop: usize,
        allowed_hops: TargetKind,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Reflect)]
pub struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn between(min: usize, max: usize) -> Self {
        Self { min, max }
    }

    fn up_to(max: usize) -> Self {
        Self::between(0, max)
    }

    fn starting_at(min: usize) -> Self {
        Self::between(min, 0)
    }
}

impl Default for Range {
    fn default() -> Self {
        Range::between(0, 1)
    }
}


// The calculation type for board searches
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[derive(Reflect)]
pub struct Scanner {
    // the unit of "stepping" for searching the board
    pub step: Step,
    // how many steps can this pattern be executed for?
    // if None, do not set a limit
    pub range: Option<Range>,
    // configuration for how to deal with colliders
    pub mode: ScanMode,
}

// TODO Chains

// notation for comments in this section often use snippets from
// https://en.wikipedia.org/wiki/Fairy_chess_piece
// in order to add additional context
impl Scanner {
    // constructors

    pub fn new(step: Step) -> Self {
        Scanner {
            step,
            ..Default::default()
        }
    }

    // "step mover" constructors
    // "Wazir", cardinal directions
    pub fn forward() -> Self {
        Scanner::new(Step::forward(1))
    }

    pub fn horizontal() -> Self {
        Scanner::new(Step::horizontal(1))
    }

    pub fn backward() -> Self {
        Scanner::new(Step::backward(1))
    }

    pub fn orthogonal() -> Self {
        Scanner::new(Step::orthogonal(1))
    }

    // "Ferz", ordinal directions
    pub fn diagonal_forward() -> Self {
        Scanner::new(Step::diagonal_forward(1))
    }

    pub fn diagonal_backward() -> Self {
        Scanner::new(Step::diagonal_backward(1))
    }

    pub fn diagonal() -> Self {
        Scanner::new(Step::diagonal(1))
    }

    // All cardinal and ordinal directions
    pub fn radial() -> Self {
        Scanner::new(Step::radial(1))
    }

    // knight jumps
    pub fn knight_rider() -> Self {
        Scanner::new(Step::leaper(2, 1))
    }

    pub fn forward_knight_rider() -> Self {
        Scanner::new(Step::forward_leaper(2, 1))
    }

    pub fn knight() -> Self {
        Scanner::knight_rider().leaper()
    }

    pub fn shogi_knight() -> Self {
        Scanner::forward_knight_rider().range(1)
    }

    // Number steps executed: leaper or rider?

    pub fn range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    pub fn leaper(self) -> Self {
        self.range(1)
    }

    pub fn rider(mut self) -> Self {
        self.range = None;
        self
    }

    // how to handle collisions

    pub fn mode(mut self, mode: ScanMode) -> Self {
        self.mode = mode;
        self
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Reflect)]
pub struct ScanTarget {
    pub target: Square,
    pub scanned_squares: Vec<Square>,
}

// Each Pattern can perform its own board search to yield a set of targetable squares
impl Scanner {
    fn get_steps(&self, orientation: Orientation) -> Vec<(i16, i16)> {
        self.step
            .movements()
            .into_iter()
            .map(|xy| orientation.orient(xy))
            .collect()
    }

    pub fn scan(
        &self,
        origin: &Square,
        orientation: Orientation,
        my_team: &Team,
        board: &Board,
        pieces: &HashMap<Square, Team>,
    ) -> Vec<ScanTarget> {
        let mut targets = Vec::new();

        for step in self.get_steps(orientation) {
            let mut scanned_squares = vec![];
            let mut steps_after_hop: Option<usize> = None;

            let board_iter = board.scan(*origin, step);
            let board_iter = match self.range {
                Some(range) => itertools::Either::Left(board_iter.take(range)),
                None => itertools::Either::Right(board_iter),
            };

            for square in board_iter {
                match self.mode {
                    ScanMode::Walk => {
                        targets.push(ScanTarget {
                            target: square,
                            scanned_squares: scanned_squares.clone(),
                        });
                        if pieces.get(&square).is_some() {
                            break;
                        }
                    }
                    ScanMode::Pierce => {
                        targets.push(ScanTarget {
                            target: square,
                            scanned_squares: scanned_squares.clone(),
                        });
                    }
                    ScanMode::Hop {
                        max_steps_after_hop,
                        allowed_hops,
                    } => {
                        if pieces
                            .get(&square)
                            .is_some_and(|target_team| allowed_hops.matches(my_team, target_team))
                        {
                            targets.push(ScanTarget {
                                target: square,
                                scanned_squares: scanned_squares.clone(),
                            });
                            steps_after_hop = Some(0);
                        } else if let Some(mut current_step_after_hop) = steps_after_hop {
                            targets.push(ScanTarget {
                                target: square,
                                scanned_squares: scanned_squares.clone(),
                            });
                            current_step_after_hop += 1;
                            if current_step_after_hop >= max_steps_after_hop {
                                break;
                            }
                        }
                    }
                }

                scanned_squares.push(square);
            }
        }

        targets
    }
}

#[cfg(test)]
mod tests {
    use crate::board::{File, Rank};

    use super::*;

    fn origin() -> Square {
        Square::new(File::C, Rank::TWO)
    }

    fn sample_board() -> HashMap<Square, Team> {
        let mut map = HashMap::new();
        map.insert(origin(), Team::White);
        map.insert(Square::new(File::B, Rank::THREE), Team::White);
        map.insert(Square::new(File::C, Rank::FIVE), Team::White);
        map.insert(Square::new(File::D, Rank::FOUR), Team::White);
        map.insert(Square::new(File::G, Rank::SIX), Team::White);
        map
    }

    #[test]
    fn bishop_on_empty_board() {
        let bishop = Scanner::diagonal();
        let results = bishop.scan(
            &origin(),
            Orientation::Up,
            &Team::White,
            &Board::chess_board(),
            &HashMap::new(),
        );
        let mut results = results.iter().map(|scan| scan.target).collect::<Vec<_>>();
        results.sort();

        let mut correct = vec![
            // up left
            Square::new(File::B, Rank::THREE),
            Square::new(File::A, Rank::FOUR),
            // up right
            Square::new(File::D, Rank::THREE),
            Square::new(File::E, Rank::FOUR),
            Square::new(File::F, Rank::FIVE),
            Square::new(File::G, Rank::SIX),
            Square::new(File::H, Rank::SEVEN),
            // down left
            Square::new(File::B, Rank::ONE),
            // down right
            Square::new(File::D, Rank::ONE),
        ];
        correct.sort();

        assert_eq!(
            results,
            correct,
            "Scanner yielded squares: {:?}",
            results
                .iter()
                .map(|square| format!("{}", square))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn bishop_on_sample_board() {
        let bishop = Scanner::diagonal();
        let results = bishop.scan(
            &origin(),
            Orientation::Up,
            &Team::White,
            &Board::chess_board(),
            &sample_board(),
        );
        let mut results = results.iter().map(|scan| scan.target).collect::<Vec<_>>();
        results.sort();

        let mut correct = vec![
            // up left
            Square::new(File::B, Rank::THREE),
            // up right
            Square::new(File::D, Rank::THREE),
            Square::new(File::E, Rank::FOUR),
            Square::new(File::F, Rank::FIVE),
            Square::new(File::G, Rank::SIX),
            // down left
            Square::new(File::B, Rank::ONE),
            // down right
            Square::new(File::D, Rank::ONE),
        ];
        correct.sort();

        assert_eq!(
            results,
            correct,
            "Scanner yielded squares: {:?}",
            results
                .iter()
                .map(|square| format!("{}", square))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn rook_on_empty_board() {
        let rook = Scanner::orthogonal();
        let results = rook.scan(
            &origin(),
            Orientation::Up,
            &Team::White,
            &Board::chess_board(),
            &HashMap::new(),
        );
        let mut results = results.iter().map(|scan| scan.target).collect::<Vec<_>>();
        results.sort();

        let mut correct = vec![
            // horizontal
            Square::new(File::C, Rank::ONE),
            Square::new(File::C, Rank::THREE),
            Square::new(File::C, Rank::FOUR),
            Square::new(File::C, Rank::FIVE),
            Square::new(File::C, Rank::SIX),
            Square::new(File::C, Rank::SEVEN),
            Square::new(File::C, Rank::EIGHT),
            // vertical
            Square::new(File::A, Rank::TWO),
            Square::new(File::B, Rank::TWO),
            Square::new(File::D, Rank::TWO),
            Square::new(File::E, Rank::TWO),
            Square::new(File::F, Rank::TWO),
            Square::new(File::G, Rank::TWO),
            Square::new(File::H, Rank::TWO),
        ];
        correct.sort();

        assert_eq!(
            results,
            correct,
            "Scanner yielded squares: {:?}",
            results
                .iter()
                .map(|square| format!("{}", square))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn rook_on_sample_board() {
        let rook = Scanner::orthogonal();
        let results = rook.scan(
            &origin(),
            Orientation::Up,
            &Team::White,
            &Board::chess_board(),
            &sample_board(),
        );
        let mut results = results.iter().map(|scan| scan.target).collect::<Vec<_>>();
        results.sort();

        let mut correct = vec![
            // horizontal
            Square::new(File::C, Rank::ONE),
            Square::new(File::C, Rank::THREE),
            Square::new(File::C, Rank::FOUR),
            Square::new(File::C, Rank::FIVE),
            // vertical
            Square::new(File::A, Rank::TWO),
            Square::new(File::B, Rank::TWO),
            Square::new(File::D, Rank::TWO),
            Square::new(File::E, Rank::TWO),
            Square::new(File::F, Rank::TWO),
            Square::new(File::G, Rank::TWO),
            Square::new(File::H, Rank::TWO),
        ];
        correct.sort();

        assert_eq!(
            results,
            correct,
            "Scanner yielded squares: {:?}",
            results
                .iter()
                .map(|square| format!("{}", square))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn knight_on_empty_board() {
        let knight = Scanner::knight();
        let results = knight.scan(
            &origin(),
            Orientation::Up,
            &Team::White,
            &Board::chess_board(),
            &HashMap::new(),
        );
        let mut results = results.iter().map(|scan| scan.target).collect::<Vec<_>>();
        results.sort();

        let mut correct = vec![
            // down left left
            Square::new(File::A, Rank::ONE),
            // up left left
            Square::new(File::A, Rank::THREE),
            // up up left
            Square::new(File::B, Rank::FOUR),
            // up up right
            Square::new(File::D, Rank::FOUR),
            // up right right
            Square::new(File::E, Rank::THREE),
            // down right right
            Square::new(File::E, Rank::ONE),
        ];
        correct.sort();

        assert_eq!(
            results,
            correct,
            "Scanner yielded squares: {:?}",
            results
                .iter()
                .map(|square| format!("{}", square))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn knight_rider_on_empty_board() {
        let knight = Scanner::knight_rider();
        let results = knight.scan(
            &origin(),
            Orientation::Up,
            &Team::White,
            &Board::chess_board(),
            &HashMap::new(),
        );
        let mut results = results.iter().map(|scan| scan.target).collect::<Vec<_>>();
        results.sort();

        let mut correct = vec![
            // down left left
            Square::new(File::A, Rank::ONE),
            // up left left
            Square::new(File::A, Rank::THREE),
            // up up left
            Square::new(File::B, Rank::FOUR),
            Square::new(File::A, Rank::SIX),
            // up up right
            Square::new(File::D, Rank::FOUR),
            Square::new(File::E, Rank::SIX),
            Square::new(File::F, Rank::EIGHT),
            // up right right
            Square::new(File::E, Rank::THREE),
            Square::new(File::G, Rank::FOUR),
            // down right right
            Square::new(File::E, Rank::ONE),
        ];
        correct.sort();

        assert_eq!(
            results,
            correct,
            "Scanner yielded squares: {:?}",
            results
                .iter()
                .map(|square| format!("{}", square))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn knight_rider_on_sample_board() {
        let knight = Scanner::knight_rider();
        let results = knight.scan(
            &origin(),
            Orientation::Up,
            &Team::White,
            &Board::chess_board(),
            &sample_board(),
        );
        let mut results = results.iter().map(|scan| scan.target).collect::<Vec<_>>();
        results.sort();

        let mut correct = vec![
            // down left left
            Square::new(File::A, Rank::ONE),
            // up left left
            Square::new(File::A, Rank::THREE),
            // up up left
            Square::new(File::B, Rank::FOUR),
            Square::new(File::A, Rank::SIX),
            // up up right
            Square::new(File::D, Rank::FOUR),
            // up right right
            Square::new(File::E, Rank::THREE),
            Square::new(File::G, Rank::FOUR),
            // down right right
            Square::new(File::E, Rank::ONE),
        ];
        correct.sort();

        assert_eq!(
            results,
            correct,
            "Scanner yielded squares: {:?}",
            results
                .iter()
                .map(|square| format!("{}", square))
                .collect::<Vec<_>>()
        );
    }
}
