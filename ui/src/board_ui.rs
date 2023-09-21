use bevy::{
    prelude::{EventWriter, Local, Query, ResMut, With},
    utils::HashMap,
};

use bevy_egui::{
    egui::{CentralPanel, RichText, Ui},
    EguiContexts,
};

use games::{
    chess::{board::Square, pieces::PieceDefinition, team::Team},
    components::{Clock, Player, Turn},
    IssueMoveEvent, IssueMutationEvent,
};

use crate::{
    icons::PieceIcon,
    mutation::IntendedMutation,
    query::{PieceData, PieceQuery},
    widgets::{BoardWidget, ClockWidget, PieceInspectorWidget, SquareWidget},
};

// TODO: custom WorldQuery to slim this fn signature
// it could use a little organization
#[allow(clippy::too_many_arguments)]
pub(crate) fn egui_chessboard(
    piece_query: Query<PieceQuery>,
    player_query: Query<(&Team, Option<&Clock>, Option<&Turn>), With<Player>>,
    mut contexts: EguiContexts,
    mut move_writer: EventWriter<IssueMoveEvent>,
    mut intended_mutation: ResMut<IntendedMutation>,
    mut mutation_writer: EventWriter<IssueMutationEvent>,
    mut last_selected_square: Local<Option<Square>>,
) {
    let team_with_turn = player_query
        .iter()
        .find_map(|(team, _, turn)| turn.map(|_| team));

    let upper_clock = player_query.iter().find_map(
        |(team, clock, _)| {
            if *team == Team::Black {
                clock
            } else {
                None
            }
        },
    );

    let bottom_clock =
        player_query.iter().find_map(
            |(team, clock, _)| {
                if *team == Team::White {
                    clock
                } else {
                    None
                }
            },
        );

    let pieces: HashMap<Square, PieceData> = piece_query
        .into_iter()
        .map(|query| (query.position.0, query.into()))
        .collect();

    CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            let mut board_selection = None;

            ui.add(BoardWidget::new(
                &pieces,
                *last_selected_square,
                &mut board_selection,
            ));

            if let Some(selected_square) = board_selection {
                // remove any mutation selection
                intended_mutation.0.take();

                if let Some(turn_event) = handle_clicked_square(
                    selected_square,
                    &mut last_selected_square,
                    &pieces,
                    team_with_turn,
                ) {
                    move_writer.send(turn_event);
                }
            }

            ui.separator();
            ui.vertical(|ui| {
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
                if let Some((_, _, icons)) = intended_mutation.0.as_ref() {
                    render_mutation_options(ui, &mut selected_mutation, icons);
                }

                if let Some(piece) = selected_mutation {
                    let (entity, action, _) = intended_mutation.0.take().unwrap();
                    mutation_writer.send(IssueMutationEvent(entity, action, piece));
                }

                if let Some(piece) = last_selected_square.and_then(|square| pieces.get(&square)) {
                    ui.add(PieceInspectorWidget::new(&piece));
                }
            });
        });
    });
}

fn handle_clicked_square(
    selected_square: Square,
    last_selected_square: &mut Option<Square>,
    pieces: &HashMap<Square, PieceData>,
    team_with_turn: Option<&Team>,
) -> Option<IssueMoveEvent> {
    if let Some(piece) = (*last_selected_square).and_then(|square| pieces.get(&square)) {
        if let Some(action) = piece.actions.get(&selected_square) {
            if let Some(team) = team_with_turn {
                if piece.team == team {
                    return Some(IssueMoveEvent(piece.entity, action.clone()));
                }
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
    piece_icons: &[(PieceIcon, PieceDefinition)],
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
