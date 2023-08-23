use bevy::{
    ecs::query::WorldQuery,
    prelude::{Entity, EventWriter, Local, Query, ResMut, With},
    utils::HashMap,
};

use bevy_egui::{
    egui::{self, Color32, Stroke, Style, Vec2, Visuals},
    EguiContexts,
};

use wildchess_game::{
    components::{
        Behavior, Pattern, PatternStep, PieceKind, Player, Position, Promotable, SearchMode,
        StartPosition, TargetMode, Targets, Team, Turn,
    },
    IssueMoveEvent, IssuePromotionEvent, Movement, Square,
};

use crate::{icons::PieceIcon, promotion::IntendedPromotion};

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
        pattern
            .range
            .map_or("".to_string(), |range| format!("up to {} squares ", range),),
        describe_step(&pattern.step),
        if pattern.search_mode == SearchMode::Walk {
            " until a collision"
        } else {
            " through any collisions"
        },
    ))
    .size(24.)
}

#[derive(WorldQuery)]
pub struct PieceQuery {
    pub entity: Entity,
    pub behavior: &'static Behavior,
    pub position: &'static Position,
    pub start_position: &'static StartPosition,
    pub team: &'static Team,
    pub targets: &'static Targets,
    pub kind: &'static PieceKind,
    pub promotable: Option<&'static Promotable>,
    pub icon: Option<&'static PieceIcon>,
}

pub struct PieceData<'a> {
    pub entity: Entity,
    pub behavior: &'a Behavior,
    pub position: &'a Position,
    pub start_position: &'a StartPosition,
    pub team: &'a Team,
    pub targets: &'a Targets,
    pub kind: &'a PieceKind,
    pub promotable: Option<&'a Promotable>,
    pub icon: Option<&'a PieceIcon>,
}

impl<'a> From<PieceQueryItem<'a>> for PieceData<'a> {
    fn from(piece: PieceQueryItem<'a>) -> Self {
        PieceData {
            entity: piece.entity,
            behavior: piece.behavior,
            position: piece.position,
            start_position: piece.start_position,
            team: piece.team,
            targets: piece.targets,
            kind: piece.kind,
            promotable: piece.promotable,
            icon: piece.icon,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn egui_chessboard(
    piece_query: Query<PieceQuery>,
    player_query: Query<&Team, (With<Player>, With<Turn>)>,
    mut contexts: EguiContexts,
    mut move_writer: EventWriter<IssueMoveEvent>,
    mut intended_promotion: ResMut<IntendedPromotion>,
    mut promotion_writer: EventWriter<IssuePromotionEvent>,
    mut selected_piece: Local<Option<Entity>>,
) {
    let Ok(team_with_turn) = player_query.get_single() else { return };

    let pieces: HashMap<Square, PieceData> = piece_query
        .into_iter()
        .map(|query| (query.position.0, query.into()))
        .collect();

    let selected_piece_entity = selected_piece.as_ref();
    let selected_piece_data = selected_piece_entity
        .and_then(|entity| piece_query.get(*entity).map(|piece| piece.position.0).ok())
        .and_then(|square| pieces.get(&square));

    let ctx = contexts.ctx_mut();
    egui::CentralPanel::default().show(&*ctx, |ui| {
        ui.horizontal(|ui| {
            egui::Grid::new("board_grid").show(ui, |ui| {
                for y in (0..=7).rev() {
                    for x in 0..=7 {
                        let square = Square::new(y.try_into().unwrap(), x.try_into().unwrap());

                        let piece = pieces
                            .get(&square)
                            .map(|piece| (piece.start_position, piece.team, piece.icon));

                        let background_color =
                            get_square_background(x, y, selected_piece_data, &square);
                        let stroke_color = get_square_stroke(selected_piece_data, &square);

                        let mut button =
                            get_button_ui(&*ctx, piece.and_then(|piece| piece.2), background_color);
                        if let Some(stroke_color) = stroke_color {
                            button = button.stroke(Stroke::new(4., stroke_color));
                        }

                        if ui.add_sized([80., 80.], button).clicked() {
                            intended_promotion.0.take();
                            handle_clicked_square(
                                square,
                                &mut selected_piece,
                                selected_piece_data,
                                &pieces,
                                &mut move_writer,
                                team_with_turn,
                            );
                        };
                    }
                    ui.end_row();
                }
            });
            ui.separator();
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(format!("{:?}'s turn.", team_with_turn)).size(36.));

                let mut promotion_behavior = None;

                if let Some((_, icons)) = intended_promotion.0.as_ref() {
                    render_promotion_buttons(ui, &*ctx, &mut promotion_behavior, icons);
                }

                if let Some(behavior) = promotion_behavior {
                    let (movement, _) = intended_promotion.0.take().unwrap();
                    promotion_writer.send(IssuePromotionEvent(movement, behavior));
                }

                if let Some(piece) = selected_piece_data {
                    render_pattern_description(ui, piece.behavior.clone(), piece.position.0);
                }
            });
        });
    });
}

const DARK_SQUARE_BG: Color32 = Color32::from_rgb(181, 136, 99);
const LIGHT_SQUARE_BG: Color32 = Color32::from_rgb(240, 217, 181);

fn get_square_background(
    x: u16,
    y: u16,
    selected_piece_data: Option<&PieceData>,
    square: &Square,
) -> Color32 {
    let (is_target_square, can_attack_square) = selected_piece_data
        .map(|piece| {
            (
                piece.targets.can_target(square),
                piece.targets.can_attack(square),
            )
        })
        .unwrap_or_else(|| (false, false));
    if is_target_square && can_attack_square {
        Color32::from_rgba_unmultiplied(180, 70, 70, 130)
    } else if is_target_square {
        Color32::from_rgba_unmultiplied(70, 70, 180, 130)
    } else if (x + y) % 2 == 0 {
        LIGHT_SQUARE_BG
    } else {
        DARK_SQUARE_BG
    }
}

fn get_square_stroke(selected_piece_data: Option<&PieceData>, square: &Square) -> Option<Color32> {
    selected_piece_data.and_then(|piece| {
        if piece.position.0 == *square {
            Some(Color32::from_rgb(140, 140, 20))
        } else {
            None
        }
    })
}

fn get_button_ui(
    context: &egui::Context,
    piece_icon: Option<&PieceIcon>,
    background_color: Color32,
) -> egui::Button {
    match piece_icon.unwrap_or(&PieceIcon::Character(' ')) {
        PieceIcon::Svg { image, .. } => {
            egui::Button::image_and_text(image.texture_id(context), Vec2::new(68., 68.), "")
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
    selected_piece_data: Option<&PieceData>,
    pieces: &HashMap<Square, PieceData>,
    move_writer: &mut EventWriter<IssueMoveEvent>,
    team_with_turn: &Team,
) {
    if let Some(piece) = selected_piece_data {
        if piece.targets.can_target(&square) && piece.team == team_with_turn {
            move_writer.send(IssueMoveEvent(Movement {
                entity: piece.entity,
                target_square: square,
            }));
            *selected_piece = None;
        } else if let Some(piece) = pieces.get(&square) {
            *selected_piece = Some(piece.entity);
        } else {
            *selected_piece = None;
        }
    } else if let Some(piece) = pieces.get(&square) {
        *selected_piece = Some(piece.entity);
    }
}

fn render_promotion_buttons(
    ui: &mut egui::Ui,
    context: &egui::Context,
    promotion_behavior: &mut Option<Behavior>,
    piece_icons: &[(PieceIcon, Behavior)],
) {
    ui.label(egui::RichText::new("Promoting! Choose a piece.").size(24.));

    ui.horizontal(|ui| {
        for (icon, behavior) in piece_icons.iter() {
            let button = get_button_ui(context, Some(icon), LIGHT_SQUARE_BG);
            if ui.add_sized([80., 80.], button).clicked() {
                *promotion_behavior = Some(behavior.clone());
            }
        }
    });

    ui.separator();
}

fn render_pattern_description(ui: &mut egui::Ui, behavior: Behavior, square: Square) {
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
