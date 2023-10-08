use bevy_egui::egui::{Color32, Response, RichText, Style, Ui, Visuals, Widget};

use games::chess::{
    pattern::{CaptureMode, CaptureRules, Pattern, RSymmetry, ScanMode, Step},
    team::Team,
};

use crate::query::PieceData;

pub struct PieceInspectorWidget<'a> {
    piece: &'a PieceData<'a>,
}

impl<'a> PieceInspectorWidget<'a> {
    pub fn new(piece: &'a PieceData<'a>) -> Self {
        Self { piece }
    }
}

impl<'a> Widget for PieceInspectorWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_style(Style {
                visuals: Visuals {
                    window_stroke: (4., Color32::WHITE).into(),
                    ..Default::default()
                },
                ..Default::default()
            });

            ui.label(RichText::new("Selected piece:").size(24.));
            if let Some(patterns) = self.piece.pattern_behavior {
                ui.label(RichText::new("Piece move patterns:").size(24.));
                for pattern in patterns.patterns.iter() {
                    ui.label(describe_pattern(pattern, self.piece.team));
                }
            }
            if let Some(relays) = self.piece.relay_behavior {
                ui.label(RichText::new("Piece relay patterns:").size(24.));
                for pattern in relays.patterns.iter() {
                    ui.label(describe_pattern(pattern, self.piece.team));
                }
            }
            if self.piece.mimic_behavior.is_some() {
                ui.label(
                    RichText::new("Piece can execute the pattern used in the last turn.").size(24.),
                );
            }
            // TODO: also consider mutations
        })
        .response
    }
}

fn describe_pattern(pattern: &Pattern, _team: &Team) -> RichText {
    RichText::new(format!(
        "- {} {}{}{}{}.",
        match pattern.capture {
            None => "move without attacking",
            Some(CaptureRules {
                mode: CaptureMode::CanCapture,
                ..
            }) => "move allowing attacks",
            Some(CaptureRules {
                mode: CaptureMode::MustCapture,
                ..
            }) => "move only to attack",
        },
        pattern
            .scanner
            .range
            .map_or("".to_string(), |range| format!("up to {} squares ", range)),
        describe_step(pattern.scanner.step.clone()),
        match pattern.scanner.mode {
            ScanMode::Walk => " until a collision",
            ScanMode::Pierce => " through any collisions",
            ScanMode::Hop { .. } => " after hopping over an enemy",
        },
        pattern
            .constraints
            .from_rank
            .as_ref()
            // TODO: use Team here
            .map_or("".to_string(), |constraint| format!(
                "when on rank {} (from its perspective)",
                char::from(&constraint.0)
            )),
    ))
    .size(24.)
}

fn describe_step(step: Step) -> String {
    match step {
        Step::OneDim(_r, symmetry) => {
            let mut directions = vec![];
            if symmetry.intersects(RSymmetry::FORWARD) {
                directions.push("forward");
            }
            if symmetry.intersects(RSymmetry::sideways()) {
                directions.push("sideways");
            }
            if symmetry.intersects(RSymmetry::BACKWARD) {
                directions.push("backward");
            }
            if symmetry.intersects(RSymmetry::diagonal()) {
                directions.push("diagonal");
            } else if symmetry.intersects(RSymmetry::diagonal_forward()) {
                directions.push("diagonal-forward");
            } else if symmetry.intersects(RSymmetry::diagonal_backward()) {
                directions.push("diagonal-backward")
            }
            format!("{}", directions.join(", "))
        }
        Step::TwoDim(a, b, _symmetry) => format!("{}-by-{}", a, b,),
    }
}
