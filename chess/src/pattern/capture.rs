use bevy::{reflect::Reflect, utils::HashMap};

use crate::{actions::Action, board::Square, team::Team};

use super::{scanner::ScanTarget, TargetKind};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub enum CaptureMode {
    #[default]
    CanCapture,
    MustCapture,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub enum CapturePattern {
    #[default]
    // captures when landing on an enemy square
    CaptureByDisplacement,
    // Captures any piece which just moved through the attacking path
    // If another piece is on the target square, the ability to move is
    // determined by `StepMode` and `CaptureTarget`
    // In a sense, this is a superset of CaptureByDisplacement
    CaptureInPassing,
    // captures when jumping over an enemy square
    // any pieces "stepped" on during traversal are captured
    CaptureByOvertake,
    // captures and does not move the piece
    CaptureAtRange,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Reflect)]
pub struct CaptureRules {
    pub mode: CaptureMode,
    pub pattern: CapturePattern,
    pub target: TargetKind,
}

impl CaptureRules {
    pub fn can_capture(&self) -> bool {
        self.mode == CaptureMode::CanCapture
    }

    pub fn must_capture(&self) -> bool {
        self.mode == CaptureMode::MustCapture
    }

    pub fn get_captures(
        &self,
        scan_target: &ScanTarget,
        my_team: &Team,
        pieces: &HashMap<Square, Team>,
        last_action: Option<&Action>,
    ) -> Vec<Square> {
        let ScanTarget {
            target,
            scanned_squares,
        } = scan_target;

        let mut capture_squares = vec![];

        let is_capturable_target = |target: &Square| {
            pieces
                .get(target)
                .is_some_and(|target_team| self.target.matches(my_team, target_team))
        };

        match self.pattern {
            CapturePattern::CaptureByDisplacement => {
                if is_capturable_target(&target) {
                    capture_squares.push(*target);
                }
            }
            CapturePattern::CaptureInPassing => {
                // this can also capture by displacement
                if is_capturable_target(&target) {
                    capture_squares.push(*target);
                }
                // and can capture in passing
                if let Some(last_action) = last_action {
                    if last_action.scanned_squares.contains(&target)
                        && is_capturable_target(&last_action.landing_square)
                    {
                        capture_squares.push(last_action.landing_square);
                    }
                }
            }
            CapturePattern::CaptureByOvertake => {
                for square in scanned_squares {
                    if is_capturable_target(square) {
                        capture_squares.push(*square);
                    }
                }
            }
            CapturePattern::CaptureAtRange => {
                if is_capturable_target(&target) {
                    capture_squares.push(*target);
                }
            }
        }

        capture_squares
    }
}
