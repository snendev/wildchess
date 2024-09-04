use itertools::Itertools;

use bevy::{
    prelude::{Entity, EventWriter, Query, Reflect, Res, ResMut, Resource},
    utils::HashMap,
};

use bevy_egui::{
    egui::{CentralPanel, Color32, RichText, ScrollArea, SidePanel, TopBottomPanel, Ui},
    EguiContexts,
};

use games::{
    chess::{board::Square, pieces::PieceDefinition, team::Team},
    components::{ActionHistory, Clock, CurrentTurn, Ply},
    RequestTurnEvent,
};
use wild_icons::PieceIconSvg;

use crate::{
    mutation::IntendedMutation,
    query::{PieceData, PieceQuery},
    widgets::{BoardWidget, ClockWidget, PieceInspectorWidget, SquareWidget},
};

#[derive(Clone, Copy, Debug, Default, Resource, Reflect)]
pub(crate) struct SelectedSquare(Option<Square>);

#[derive(Clone, Copy, Debug, Default, Resource, Reflect)]
pub(crate) struct SelectedHistoricalPly(Option<Ply>);

#[derive(Clone, Copy, Debug, Default, Resource, Reflect)]
pub(crate) struct SelectedGame(pub Option<Entity>);

pub(crate) fn egui_history_panel(
    mut contexts: EguiContexts,
    games_query: Query<&ActionHistory>,
    selected_game: Res<SelectedGame>,
    mut selected_ply: ResMut<SelectedHistoricalPly>,
) {
    let Some(history) = selected_game.0.and_then(|game| games_query.get(game).ok()) else {
        return;
    };
    let total_count = history.len();

    TopBottomPanel::top("Move history")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.label(RichText::new("Moves").size(36.));
            ui.add_space(20.);
            ui.set_min_height(300.);
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(280.)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.set_width(150.);
                        for chunk in &history.iter().enumerate().chunks(2) {
                            // for now there are only two players
                            ui.horizontal(|ui| {
                                for (index, (_entity, action)) in chunk.into_iter() {
                                    let move_text = format!(
                                        "{}{}{}",
                                        action.movement.from,
                                        if action.captures.is_empty() { "-" } else { "x" },
                                        action.movement.to,
                                    );
                                    let text = RichText::new(move_text)
                                        .size(28.)
                                        .strong()
                                        .color(Color32::BLACK);
                                    let is_current_ply = index == total_count - 1;
                                    let selected = selected_ply
                                        .0
                                        .is_some_and(|ply| ply == Ply::new(index + 1))
                                        || (selected_ply.0.is_none() && is_current_ply);
                                    if ui.selectable_label(selected, text).clicked() && !selected {
                                        selected_ply.0 = if is_current_ply {
                                            None
                                        } else {
                                            Some(Ply::new(index + 1))
                                        }
                                    };
                                }
                            });
                        }
                    });
                });
            ui.add_space(20.);

            ui.horizontal(|ui| {
                if ui
                    .button(RichText::new("<").size(32.).strong().color(Color32::BLACK))
                    .clicked()
                {
                    if let Some(ply) = &mut selected_ply.0 {
                        ply.decrement();
                    } else if total_count > 0 {
                        selected_ply.0 = Some(Ply::new(total_count - 1));
                    }
                }
                if ui
                    .button(RichText::new(">").size(32.).strong().color(Color32::BLACK))
                    .clicked()
                {
                    if selected_ply
                        .0
                        .is_some_and(|ply| ply < Ply::new(total_count - 1))
                    {
                        selected_ply.0.as_mut().unwrap().increment();
                    } else {
                        selected_ply.0 = None;
                    }
                }
            });

            ui.add_space(20.);
        });
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn egui_information_panel(
    mut contexts: EguiContexts,
    game_query: Query<&CurrentTurn>,
    piece_query: Query<PieceQuery>,
    player_query: Query<(&Team, Option<&Clock>)>,
    mut mutation_writer: EventWriter<RequestTurnEvent>,
    mut intended_mutation: ResMut<IntendedMutation>,
    selected_square: Res<SelectedSquare>,
    selected_game: Res<SelectedGame>,
) {
    let Some(current_game) = selected_game.0 else {
        return;
    };
    let Ok(team_with_turn) = game_query.get(current_game) else {
        return;
    };

    let upper_clock = player_query.iter().find_map(
        |(team, clock)| {
            if *team == Team::Black {
                clock
            } else {
                None
            }
        },
    );

    let bottom_clock =
        player_query.iter().find_map(
            |(team, clock)| {
                if *team == Team::White {
                    clock
                } else {
                    None
                }
            },
        );

    let pieces: HashMap<Square, PieceData> = piece_query
        .into_iter()
        .filter_map(|item| {
            if item.in_game.0 == current_game && item.position.is_some() {
                Some((item.position.unwrap().0, item.into()))
            } else {
                None
            }
        })
        .collect();

    CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                if let Some(clock) = upper_clock {
                    ui.add(ClockWidget::new(clock));
                }
                ui.add_space(16.);
                if let Some(clock) = bottom_clock {
                    ui.add(ClockWidget::new(clock));
                }
                ui.add_space(100.);

                ui.label(RichText::new(format!("{:?}'s turn.", team_with_turn)).size(36.));

                let mut selected_mutation = None;
                if let Some((_, icons)) = intended_mutation.0.as_ref() {
                    render_mutation_options(ui, &mut selected_mutation, icons);
                }

                if let Some(piece_definition) = selected_mutation {
                    let (event, _) = intended_mutation.0.take().unwrap();
                    mutation_writer.send(RequestTurnEvent::new_with_mutation(
                        event.piece,
                        event.game,
                        event.action,
                        piece_definition,
                    ));
                }

                if let Some(piece) = selected_square.0.and_then(|square| pieces.get(&square)) {
                    ui.add(PieceInspectorWidget::new(piece));
                }
            });
    });
}

// TODO: custom WorldQuery to slim this fn signature
// it could use a little organization
#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn egui_chessboard(
    mut contexts: EguiContexts,
    game_query: Query<&CurrentTurn>,
    piece_query: Query<PieceQuery>,
    mut move_writer: EventWriter<RequestTurnEvent>,
    mut intended_mutation: ResMut<IntendedMutation>,
    mut last_selected_square: ResMut<SelectedSquare>,
    selected_game: Res<SelectedGame>,
    selected_ply: Res<SelectedHistoricalPly>,
) {
    let Some(current_game) = selected_game.0 else {
        return;
    };
    let Ok(team_with_turn) = game_query.get(current_game) else {
        return;
    };

    let pieces: HashMap<Square, PieceData> = piece_query
        .into_iter()
        .map(|item| {
            if let Some(ply) = selected_ply.0 {
                item.to_historical_piece_data(&ply)
            } else {
                item.into()
            }
        })
        .filter_map(|item| {
            if item.in_game.0 == current_game && item.position.is_some() {
                Some((item.position.unwrap().0, item))
            } else {
                None
            }
        })
        .collect();

    let selected_square = if selected_ply.0.is_some() {
        None
    } else {
        last_selected_square.0
    };

    SidePanel::left("chessboard")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            let mut board_selection = None;

            ui.add(BoardWidget::new(
                &pieces,
                selected_square,
                &mut board_selection,
            ));

            if selected_ply.0.is_none() {
                if let Some(selected_square) = board_selection {
                    // remove any mutation selection
                    intended_mutation.0.take();

                    if let Some(turn_event) = handle_clicked_square(
                        selected_square,
                        current_game,
                        &mut last_selected_square.0,
                        &pieces,
                        team_with_turn.0,
                    ) {
                        move_writer.send(turn_event);
                    }
                }
            }
        });
}

fn handle_clicked_square(
    selected_square: Square,
    current_game: Entity,
    last_selected_square: &mut Option<Square>,
    pieces: &HashMap<Square, PieceData>,
    team_with_turn: Team,
) -> Option<RequestTurnEvent> {
    if let Some(piece) = (*last_selected_square).and_then(|square| pieces.get(&square)) {
        if let Some(action) = piece.actions.get(&selected_square) {
            if *piece.team == team_with_turn {
                *last_selected_square = None;
                return Some(RequestTurnEvent::new(
                    piece.entity,
                    current_game,
                    action.clone(),
                ));
            }
        }
    }
    if pieces.get(&selected_square).is_some() {
        *last_selected_square = Some(selected_square);
    }
    None
}

fn render_mutation_options(
    ui: &mut Ui,
    selected_mutation: &mut Option<PieceDefinition>,
    piece_icons: &[(PieceIconSvg, PieceDefinition)],
) {
    ui.label(RichText::new("Promoting! Choose a piece.").size(24.));

    ui.horizontal(|ui| {
        for (icon, behavior) in piece_icons.iter() {
            let square = Square::default();
            if ui
                .add(SquareWidget::new_from_context(square, Some(icon), None))
                .clicked()
            {
                *selected_mutation = Some(behavior.clone());
            }
        }
    });

    ui.separator();
}
