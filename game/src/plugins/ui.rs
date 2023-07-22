use anyhow::{Error as AnyError, Result as AnyResult};
use thiserror::Error;

use bevy::{
    prelude::{Entity, EventWriter, Local, Query, Res},
    utils::HashMap,
};

use bevy_egui::{
    egui::{self, Color32, Style, Visuals},
    EguiContexts,
};

use crate::{
    behavior::PatternStep,
    board::{GamePieces, King, Pawn, PieceIdentity},
    Behavior, MovePieceEvent, Pattern, Promotion, Rank, SearchMode, Square, TargetMode,
    Team::{self, Black, White},
    Vision,
};

fn piece_icon(id: PieceIdentity, team: Team) -> char {
    match (id, team) {
        (PieceIdentity::AH, White) => '\u{2661}',
        (PieceIdentity::AH, Black) => '\u{2665}',
        (PieceIdentity::BG, White) => '\u{2740}',
        (PieceIdentity::BG, Black) => '\u{2663}',
        (PieceIdentity::CF, White) => '\u{26C4}',
        (PieceIdentity::CF, Black) => '\u{2603}',
        (PieceIdentity::D, White) => '\u{2606}',
        (PieceIdentity::D, Black) => '\u{2605}',
    }
}

#[derive(Debug, Error)]
#[error("Piece must be only one of piece, pawn, or king.")]
struct InvalidPieceError;

fn wild_piece_unicode(
    (king, pawn, piece): (Option<King>, Option<Pawn>, Option<PieceIdentity>),
    team: Team,
) -> AnyResult<char> {
    match (king, pawn, piece, team) {
        (Some(King), None, None, White) => Ok('\u{2654}'),
        (Some(King), None, None, Black) => Ok('\u{265A}'),
        (None, Some(Pawn), None, White) => Ok('\u{2659}'),
        (None, Some(Pawn), None, Black) => Ok('\u{265F}'),
        (None, None, Some(id), team) => Ok(piece_icon(id, team)),
        (None, None, None, White) => Ok('W'),
        (None, None, None, Black) => Ok('B'),
        _ => Err(AnyError::new(InvalidPieceError)),
    }
}

fn describe_step(step: &PatternStep) -> String {
    match step {
        PatternStep { x: 0, y: 1 } => "forward".to_string(),
        PatternStep { x: 1, y: 0 } => "sideways".to_string(),
        PatternStep { x: 0, y: -1 } => "backward".to_string(),
        PatternStep { x: 1, y: 1 } => "diagonally-forward".to_string(),
        PatternStep { x: 1, y: -1 } => "diagonally-backward".to_string(),
        PatternStep { x, y } => format!("{}-by-{}", x, y,),
    }
}

fn describe_pattern(pattern: &Pattern) -> egui::RichText {
    egui::RichText::new(format!(
        "- {} {}{}{}.",
        match pattern.target_mode {
            TargetMode::Moving => "move without attacking",
            TargetMode::Attacking => "move allowing attacks",
            TargetMode::OnlyAttacking => "move only to attack",
        },
        pattern.range.map_or("".to_string(), |range| format!(
            "up to {} squares ",
            range.to_string()
        ),),
        describe_step(&pattern.step),
        if pattern.search_mode == SearchMode::Walk {
            " until a collision"
        } else {
            " through any collisions"
        },
    ))
    .size(24.)
}

pub type PieceQuery<'a> = (
    Entity,
    &'a Behavior,
    &'a Square,
    &'a Team,
    &'a Vision,
    (
        Option<&'a King>,
        Option<&'a Pawn>,
        Option<&'a PieceIdentity>,
    ),
);
pub type PieceTuple = (
    Entity,
    Behavior,
    Square,
    Team,
    Vision,
    (Option<King>, Option<Pawn>, Option<PieceIdentity>),
);
pub fn egui_chessboard(
    query: Query<PieceQuery>,
    mut contexts: EguiContexts,
    mut writer: EventWriter<MovePieceEvent>,
    game_pieces: Res<GamePieces>,
    mut selected_piece: Local<Option<Entity>>,
    mut intended_promotion: Local<Option<(Entity, Square, Team)>>,
) {
    let pieces: HashMap<Square, PieceTuple> = query
        .iter()
        .map(
            |(entity, behavior, square, team, vision, (king, pawn, piece))| {
                (
                    square.clone(),
                    (
                        entity,
                        behavior.clone(),
                        square.clone(),
                        *team,
                        vision.clone(),
                        (king.map(|k| *k), pawn.map(|p| *p), piece.map(|p| *p)),
                    ),
                )
            },
        )
        .collect();

    let selected_piece_entity = selected_piece.as_ref();
    let selected_piece_data = selected_piece_entity
        .map(|entity| {
            let square = query
                .get(*entity)
                .map(|(_, _, square, _, _, _)| square.clone())
                .ok();
            square.map(|square| pieces.get(&square)).flatten()
        })
        .flatten();

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            egui::Grid::new("board_grid").show(ui, |ui| {
                for y in (0..=7).rev() {
                    for x in 0..=7 {
                        let square = Square::new(x.try_into().unwrap(), y.try_into().unwrap());

                        let (is_target_square, can_attack_square) = selected_piece_data
                            .map(|(_, _, _, _, vision, _)| {
                                (vision.can_target(&square), vision.can_attack(&square))
                            })
                            .unwrap_or_else(|| (false, false));

                        let text = if let Some((_, _, _, team, _, piece)) = pieces.get(&square) {
                            egui::RichText::new(wild_piece_unicode(*piece, *team).unwrap())
                                .color(match *team {
                                    Team::White => Color32::LIGHT_GRAY,
                                    Team::Black => Color32::WHITE,
                                })
                                .size(18.)
                        } else {
                            egui::RichText::new("-").size(18.)
                        };
                        let button = ui.add_sized(
                            [80., 80.],
                            egui::Button::new(text).fill(
                                if is_target_square && can_attack_square {
                                    Color32::from_rgba_unmultiplied(139, 0, 0, 32)
                                } else if is_target_square {
                                    Color32::from_rgba_unmultiplied(0, 0, 139, 32)
                                } else if (x + y) % 2 == 0 {
                                    Color32::from_rgb(64, 64, 64)
                                } else {
                                    Color32::from_rgb(32, 32, 32)
                                },
                            ),
                        );
                        if button.clicked() {
                            if let Some((entity, _, _, team, vision, piece)) = selected_piece_data {
                                if vision.can_target(&square) {
                                    if piece.1.is_some()
                                        && match (team, square.rank) {
                                            (Team::White, Rank::Eight)
                                            | (Team::Black, Rank::One) => true,
                                            _ => false,
                                        }
                                    {
                                        *intended_promotion = Some((*entity, square, *team));
                                    } else {
                                        writer.send(MovePieceEvent(*entity, square, None));
                                    }
                                    *selected_piece = None;
                                } else if let Some((current_entity, _, _, _, _, _)) =
                                    pieces.get(&square)
                                {
                                    *selected_piece = Some(*current_entity);
                                } else {
                                    *selected_piece = None;
                                }
                            } else if let Some((current_entity, _, _, _, _, _)) =
                                pieces.get(&square)
                            {
                                *selected_piece = Some(*current_entity);
                            }
                        };
                    }
                    ui.end_row();
                }
            });
            ui.separator();
            ui.vertical(|ui| {
                let mut promotion_id = None;
                if let Some((_, _, team)) = intended_promotion.as_ref() {
                    ui.label(egui::RichText::new(format!("Promoting! Choose a piece.")).size(24.));

                    ui.horizontal(|ui| {
                        for (id, _) in game_pieces.0.iter() {
                            let icon = egui::RichText::new(
                                wild_piece_unicode((None, None, Some(*id)), *team).unwrap(),
                            )
                            .color(match team {
                                Team::White => Color32::LIGHT_GRAY,
                                Team::Black => Color32::WHITE,
                            })
                            .size(18.);
                            let button = ui.add_sized(
                                [80., 80.],
                                egui::Button::new(icon).fill(Color32::from_rgb(64, 64, 64)),
                            );
                            if button.clicked() {
                                promotion_id = Some(*id);
                            }
                        }
                    });

                    ui.separator();
                }

                if let Some(promotion_id) = promotion_id {
                    let (entity, square, _) = intended_promotion.take().unwrap();
                    writer.send(MovePieceEvent(
                        entity,
                        square.clone(),
                        Some(Promotion::to(promotion_id)),
                    ));
                }

                if let Some((_, behavior, square, _, _, _)) = selected_piece_data {
                    ui.set_style(Style {
                        visuals: Visuals {
                            window_stroke: (4., Color32::WHITE).into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    });

                    ui.label(
                        egui::RichText::new(format!("Selected piece: {:?}", square)).size(24.),
                    );
                    ui.label(egui::RichText::new("Piece move patterns:").size(24.));
                    for pattern in behavior.patterns.iter() {
                        ui.label(describe_pattern(pattern));
                    }
                }
            });
        })
    });
}
