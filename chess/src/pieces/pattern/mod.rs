use itertools::Either;

use bevy::{prelude::Reflect, utils::HashMap};

use crate::{
    board::{Rank, Square},
    pieces::{Action, Orientation},
    team::Team,
};

mod capture;
pub use capture::{CaptureMode, CapturePattern, CaptureRules};
mod step;
pub use step::{ABSymmetry, RSymmetry, Step};
mod targets;
pub use targets::TargetKind;
mod scanner;
pub use scanner::{ScanMode, ScanTarget, Scanner};

// The calculation type for board searches
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct Pattern {
    // struct that defines how to walk the board space
    pub scanner: Scanner,
    // TODO: move these two onto wrapper?
    // when Some, this enables capturing pieces when executing this pattern
    pub capture: Option<CaptureRules>,
    // which squares this pattern can be activated from, if any
    pub constraints: Constraints,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct Constraints {
    pub from_rank: Option<FromRankConstraint>,
    pub forbidden_targets: Option<ForbiddenTargetConstraint>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct FromRankConstraint(Rank);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct ForbiddenTargetConstraint(Vec<Square>);

// TODO Chains

// notation for comments in this section often use snippets from
// https://en.wikipedia.org/wiki/Fairy_chess_piece
// in order to add additional context
impl Pattern {
    // constructors

    pub fn new(step: Step) -> Self {
        Pattern {
            scanner: Scanner::new(step),
            ..Default::default()
        }
    }

    // "step mover" constructors
    // "Wazir", cardinal directions
    pub fn forward() -> Self {
        Pattern::new(Step::forward(1))
    }

    pub fn horizontal() -> Self {
        Pattern::new(Step::horizontal(1))
    }

    pub fn backward() -> Self {
        Pattern::new(Step::backward(1))
    }

    pub fn orthogonal() -> Self {
        Pattern::new(Step::orthogonal(1))
    }

    // "Ferz", ordinal directions
    pub fn diagonal_forward() -> Self {
        Pattern::new(Step::diagonal_forward(1))
    }

    pub fn diagonal_backward() -> Self {
        Pattern::new(Step::diagonal_backward(1))
    }

    pub fn diagonal() -> Self {
        Pattern::new(Step::diagonal(1))
    }

    // All cardinal and ordinal directions
    pub fn radial() -> Self {
        Pattern::new(Step::radial(1))
    }

    // knight jumps
    pub fn knight() -> Self {
        Pattern::new(Step::leaper(2, 1))
    }

    pub fn shogi_knight() -> Self {
        Pattern::new(Step::forward_leaper(2, 1))
    }

    // classical en passant
    // (N.B. this only describes the attack pattern, and does not take into account
    // whether the target piece is a pawn)
    pub fn en_passant() -> Self {
        Pattern {
            scanner: Scanner::new(Step::diagonal_forward(1)).range(1),
            capture: Some(CaptureRules {
                mode: CaptureMode::MustCapture,
                pattern: CapturePattern::CaptureInPassing,
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    // Number steps executed: leaper or rider?

    pub fn range(mut self, range: u8) -> Self {
        self.scanner = self.scanner.range(range);
        self
    }

    pub fn leaper(mut self) -> Self {
        self.scanner = self.scanner.leaper();
        self
    }

    pub fn rider(mut self) -> Self {
        self.scanner = self.scanner.rider();
        self
    }

    // common capture rules

    pub fn captures_by_displacement(mut self) -> Self {
        self.capture = Some(CaptureRules {
            mode: CaptureMode::CanCapture,
            pattern: CapturePattern::CaptureByDisplacement,
            target: TargetKind::Enemy,
        });
        self
    }

    pub fn only_captures_by_displacement(mut self) -> Self {
        self.capture = Some(CaptureRules {
            mode: CaptureMode::MustCapture,
            pattern: CapturePattern::CaptureByDisplacement,
            target: TargetKind::Enemy,
        });
        self
    }

    pub fn captures_by_overtake(mut self) -> Self {
        self.capture = Some(CaptureRules {
            mode: CaptureMode::MustCapture,
            pattern: CapturePattern::CaptureByOvertake,
            target: TargetKind::Enemy,
        });
        self
    }

    // scan mode
    pub fn scan_mode(mut self, mode: ScanMode) -> Self {
        self.scanner.mode = mode;
        self
    }

    // common constraints

    pub fn only_from_local_rank(mut self, rank: Rank) -> Self {
        self.constraints.from_rank = Some(FromRankConstraint(rank));
        self
    }

    pub fn forbidden_from_squares(mut self, squares: Vec<Square>) -> Self {
        self.constraints.forbidden_targets = Some(ForbiddenTargetConstraint(squares));
        self
    }
}

// Each Pattern can perform its own search and yield a set of squares
impl Pattern {
    fn get_action_for_target(
        &self,
        scan_target: ScanTarget,
        origin: &Square,
        orientation: &Orientation,
        my_team: &Team,
        pieces: &HashMap<Square, Team>,
        last_action: Option<&Action>,
    ) -> Option<(Square, Action)> {
        let colliding_piece = pieces.get(&scan_target.target);

        if let Some(capture) = self.capture {
            let captures = capture.get_captures(&scan_target, my_team, pieces, last_action);
            if capture.must_capture() && captures.is_empty() {
                None
            } else if capture.pattern != CapturePattern::CaptureByDisplacement
                && colliding_piece.is_some()
            {
                None
            } else if colliding_piece.is_some_and(|team| !capture.target.matches(my_team, team)) {
                None
            } else {
                let landing_square = match capture.pattern {
                    CapturePattern::CaptureAtRange => *origin,
                    _ => scan_target.target,
                };
                Some((
                    landing_square,
                    // N.B. not always actually a capture, if captures is empty
                    Action::capture(
                        landing_square,
                        *orientation,
                        scan_target.scanned_squares,
                        self.clone(),
                        captures,
                    ),
                ))
            }
        } else if colliding_piece.is_some() {
            None
        } else {
            Some((
                scan_target.target,
                Action::movement(
                    scan_target.target,
                    *orientation,
                    scan_target.scanned_squares,
                    self.clone(),
                ),
            ))
        }
    }

    pub fn search(
        &self,
        origin: &Square,
        orientation: &Orientation,
        my_team: &Team,
        board_max: &Square,
        pieces: &HashMap<Square, Team>,
        last_action: Option<&Action>,
    ) -> HashMap<Square, Action> {
        if let Some(rank_constraint) = &self.constraints.from_rank {
            if origin.rank != rank_constraint.0 {
                return HashMap::new();
            }
        }

        let scan_targets = self
            .scanner
            .scan(origin, *orientation, my_team, board_max, pieces);

        if let Some(ForbiddenTargetConstraint(forbidden_squares)) =
            &self.constraints.forbidden_targets
        {
            Either::Right(scan_targets.into_iter().filter_map(|target| {
                if forbidden_squares.contains(&target.target) {
                    None
                } else {
                    Some(target)
                }
            }))
        } else {
            Either::Left(scan_targets.into_iter())
        }
        // TODO: how do squares with multiple actions merge them?
        // this is fine for now but we can do better
        .filter_map(|scan_target| {
            self.get_action_for_target(
                scan_target,
                origin,
                orientation,
                my_team,
                pieces,
                last_action,
            )
        })
        .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::board::{common::chess_board, File};

    use super::*;

    fn origin() -> Square {
        Square::new(File::C, Rank::TWO)
    }

    fn sample_board() -> HashMap<Square, Team> {
        let mut map = HashMap::new();
        map.insert(origin(), Team::White);
        map.insert(Square::new(File::B, Rank::THREE), Team::White);
        map.insert(Square::new(File::C, Rank::FIVE), Team::Black);
        map.insert(Square::new(File::D, Rank::FOUR), Team::Black);
        map.insert(Square::new(File::G, Rank::SIX), Team::Black);
        map
    }

    #[test]
    fn bishop_pattern_on_empty_board() {
        let bishop = Pattern::diagonal().captures_by_displacement();
        let results = bishop.search(
            &origin(),
            &Orientation::Up,
            &Team::White,
            &chess_board().size,
            &HashMap::new(),
            None,
        );
        let mut results = results
            .iter()
            .map(|(square, _)| *square)
            .collect::<Vec<_>>();
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
                .map(|square| format!("{} ", square))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn bishop_pattern_on_sample_board() {
        let bishop = Pattern::diagonal().captures_by_displacement();
        let results = bishop.search(
            &origin(),
            &Orientation::Up,
            &Team::White,
            &chess_board().size,
            &sample_board(),
            None,
        );
        let capture_square = Square::new(File::G, Rank::SIX);
        assert_eq!(
            results
                .get(&capture_square)
                .and_then(|action| action.captures.first()),
            Some(&capture_square),
            "c2 Bishop cannot capture enemy piece on g6 when it should!",
        );
        let mut results = results
            .iter()
            .map(|(square, _)| *square)
            .collect::<Vec<_>>();
        results.sort();

        let mut correct = vec![
            // colliding white piece on b3 stops up-left
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
}
