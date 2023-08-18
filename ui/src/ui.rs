use bevy::{
    prelude::{Entity, EventReader, EventWriter, Local, Query, Res, ResMut, Resource},
    utils::HashMap,
};

use bevy_egui::{
    egui::{self, Color32, Stroke, Style, Vec2, Visuals},
    EguiContexts,
};

use wildchess_game::{
    components::{
        Behavior, Pattern, PatternStep, PieceKind, Position, SearchMode, StartPosition, TargetMode,
        Targets, Team,
    },
    GamePieces, Movement, PieceConfiguration, PieceEvent, Promotion, RequestPromotion, Square,
};

use crate::icons::{PieceIcon, PieceIcons};

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

#[derive(Default, Resource)]
pub struct IntendedPromotion(Option<RequestPromotion>);

pub fn read_promotions(
    mut intended_promotion: ResMut<IntendedPromotion>,
    mut promotion_reader: EventReader<PieceEvent<RequestPromotion>>,
) {
    for event in promotion_reader.iter() {
        intended_promotion.0 = Some(event.get().clone());
    }
}

pub type PieceQuery<'a> = (
    Entity,
    &'a Behavior,
    &'a Position,
    &'a StartPosition,
    &'a Team,
    &'a Targets,
    &'a PieceKind,
);
pub type PieceTuple = (
    Entity,
    Behavior,
    Position,
    StartPosition,
    Team,
    Targets,
    PieceKind,
);
pub fn egui_chessboard(
    query: Query<PieceQuery>,
    mut contexts: EguiContexts,
    mut move_writer: EventWriter<PieceEvent<Movement>>,
    mut promotion_writer: EventWriter<PieceEvent<Promotion>>,
    mut intended_promotion: ResMut<IntendedPromotion>,
    game_pieces: Res<GamePieces>,
    piece_icons: Res<PieceIcons>,
    mut selected_piece: Local<Option<Entity>>,
) {
    let pieces: HashMap<Square, PieceTuple> = query
        .iter()
        .map(
            |(entity, behavior, position, start_position, team, vision, piece)| {
                (
                    position.0,
                    (
                        entity,
                        behavior.clone(),
                        position.clone(),
                        start_position.clone(),
                        *team,
                        vision.clone(),
                        *piece,
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
                .map(|(_, _, position, _, _, _, _)| position.0)
                .ok();
            square.map(|square| pieces.get(&square)).flatten()
        })
        .flatten();

    let ctx = contexts.ctx_mut();
    egui::CentralPanel::default().show(&*ctx, |ui| {
        ui.horizontal(|ui| {
            egui::Grid::new("board_grid").show(ui, |ui| {
                for y in (0..=7).rev() {
                    for x in 0..=7 {
                        let square = Square::new(y.try_into().unwrap(), x.try_into().unwrap());

                        let piece = pieces
                            .get(&square)
                            .map(|(_, _, _, start_position, team, _, _)| (start_position, team));

                        let background_color =
                            get_square_background(x, y, selected_piece_data, &square);
                        let stroke_color = get_square_stroke(selected_piece_data, &square);

                        let mut button =
                            get_button(&*ctx, piece, piece_icons.as_ref(), background_color);
                        if let Some(stroke_color) = stroke_color {
                            button = button.stroke(Stroke::new(4., stroke_color));
                        }

                        if ui.add_sized([80., 80.], button).clicked() {
                            handle_clicked_square(
                                square,
                                &mut selected_piece,
                                selected_piece_data,
                                &pieces,
                                &mut move_writer,
                            );
                        };
                    }
                    ui.end_row();
                }
            });
            ui.separator();
            ui.vertical(|ui| {
                let mut promotion_behavior = None;

                if let Some(promotion) = intended_promotion.0.as_ref() {
                    if let Ok((_, _, _, _, team, _, _)) = query.get(promotion.entity()) {
                        render_promotion_buttons(
                            ui,
                            &*ctx,
                            &mut promotion_behavior,
                            piece_icons.as_ref(),
                            game_pieces.as_ref(),
                            team,
                        );
                    }
                }

                if let Some(behavior) = promotion_behavior {
                    let promotion_request = intended_promotion.0.take().unwrap();
                    promotion_writer.send(PieceEvent::<Promotion>::new(
                        promotion_request.promote(behavior),
                    ));
                }

                render_pattern_description(ui, selected_piece_data);
            });
        })
    });
}

const DARK_SQUARE_BG: Color32 = Color32::from_rgb(181, 136, 99);
const LIGHT_SQUARE_BG: Color32 = Color32::from_rgb(240, 217, 181);

fn get_square_background(
    x: u8,
    y: u8,
    selected_piece_data: Option<&PieceTuple>,
    square: &Square,
) -> Color32 {
    let (is_target_square, can_attack_square) = selected_piece_data
        .map(|(_, _, _, _, _, vision, _)| (vision.can_target(square), vision.can_attack(square)))
        .unwrap_or_else(|| (false, false));
    if is_target_square && can_attack_square {
        Color32::from_rgba_unmultiplied(139, 0, 0, 32)
    } else if is_target_square {
        Color32::from_rgba_unmultiplied(0, 0, 139, 32)
    } else if (x + y) % 2 == 0 {
        LIGHT_SQUARE_BG
    } else {
        DARK_SQUARE_BG
    }
}

fn get_square_stroke(selected_piece_data: Option<&PieceTuple>, square: &Square) -> Option<Color32> {
    selected_piece_data.and_then(|(_, _, position, _, _, _, _)| {
        if position.0 == *square {
            Some(Color32::from_rgb(140, 140, 20))
        } else {
            None
        }
    })
}

fn get_button(
    context: &egui::Context,
    piece: Option<(&StartPosition, &Team)>,
    piece_icons: &PieceIcons,
    background_color: Color32,
) -> egui::Button {
    match piece
        .and_then(|(start_position, team)| {
            piece_icons
                .0
                .get(&(start_position.clone(), *team))
                .map(|icon| icon)
        })
        .unwrap_or(&PieceIcon::Character('-'))
    {
        PieceIcon::Svg(icon) => {
            egui::Button::image_and_text(icon.texture_id(context), Vec2::new(68., 68.), "")
                .fill(background_color)
        }
        PieceIcon::Character(character) => {
            let text = egui::RichText::new(*character)
                .size(18.)
                .color(Color32::BLACK);
            egui::Button::new(text).fill(background_color)
        }
    }
}

fn handle_clicked_square(
    square: Square,
    selected_piece: &mut Option<Entity>,
    selected_piece_data: Option<&PieceTuple>,
    pieces: &HashMap<Square, PieceTuple>,
    move_writer: &mut EventWriter<PieceEvent<Movement>>,
) {
    if let Some((entity, _, _, _, _, vision, _)) = selected_piece_data {
        if vision.can_target(&square) {
            move_writer.send(PieceEvent::<Movement>::new(*entity, square));
            *selected_piece = None;
        } else if let Some((current_entity, _, _, _, _, _, _)) = pieces.get(&square) {
            *selected_piece = Some(*current_entity);
        } else {
            *selected_piece = None;
        }
    } else if let Some((current_entity, _, _, _, _, _, _)) = pieces.get(&square) {
        *selected_piece = Some(*current_entity);
    }
}

fn render_promotion_buttons(
    ui: &mut egui::Ui,
    context: &egui::Context,
    promotion_behavior: &mut Option<Behavior>,
    piece_icons: &PieceIcons,
    game_pieces: &GamePieces,
    team: &Team,
) {
    ui.label(egui::RichText::new(format!("Promoting! Choose a piece.")).size(24.));

    ui.horizontal(|ui| {
        for (PieceConfiguration { behavior, .. }, start_positions) in game_pieces.0.iter() {
            let start_position = start_positions.first().unwrap();
            let button = get_button(
                context,
                Some((start_position, team)),
                piece_icons,
                LIGHT_SQUARE_BG,
            );
            if ui.add_sized([80., 80.], button).clicked() {
                *promotion_behavior = Some(behavior.clone());
            }
        }
    });

    ui.separator();
}

fn render_pattern_description(ui: &mut egui::Ui, selected_piece_data: Option<&PieceTuple>) {
    if let Some((_, behavior, square, _, _, _, _)) = selected_piece_data {
        ui.set_style(Style {
            visuals: Visuals {
                window_stroke: (4., Color32::WHITE).into(),
                ..Default::default()
            },
            ..Default::default()
        });

        ui.label(egui::RichText::new(format!("Selected piece: {:?}", square)).size(24.));
        ui.label(egui::RichText::new("Piece move patterns:").size(24.));
        for pattern in behavior.patterns.iter() {
            ui.label(describe_pattern(pattern));
        }
    }
}
