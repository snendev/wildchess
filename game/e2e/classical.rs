use anyhow::Result as AnyResult;

use bevy::{
    prelude::{Commands, Component, Entity, EventWriter, Local, Query, Startup, Update},
    utils::HashMap,
};
use bevy_egui::{
    egui::{self, Color32},
    EguiContexts,
};

use bevy_geppetto::Test;

use wildchess_game::{
    pieces::{PieceBundle, PieceKind},
    Behavior, File, GameplayPlugin, MovePieceEvent, Pattern, Rank, Square,
    Team::{self, Black, White},
    Vision,
};

fn main() {
    Test {
        label: "test classical board".to_string(),
        setup: |app| {
            app.add_plugins(GameplayPlugin)
                .add_systems(Startup, classical_board)
                .add_systems(Update, classical_ui);
        },
    }
    .run()
}

#[derive(Clone, Copy, Component)]
pub enum PieceIcon {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

// no en passent, no "two moves forward" rule
pub fn pawn(team: Team, file: File, rank: Rank) -> PieceBundle {
    PieceBundle {
        kind: PieceKind::Pawn,
        behavior: Behavior::default()
            .with_pattern(Pattern::forward().range(1).cannot_attack())
            .with_pattern(Pattern::diagonal_forward().range(1).can_attack()),
        square: Square::new(file, rank),
        team,
        vision: Vision::default(),
    }
}

// no castling
fn king(team: Team, rank: Rank) -> PieceBundle {
    PieceBundle {
        behavior: Behavior::builder().radials().range(1).can_attack().build(),
        kind: PieceKind::King,
        square: Square::new(File::E, rank),
        team,
        vision: Vision::default(),
    }
}

fn knight(team: Team, file: File, rank: Rank) -> AnyResult<PieceBundle> {
    Ok(PieceBundle {
        kind: PieceKind::SquareBG,
        behavior: Behavior::builder()
            .knight_jumps()
            .range(1)
            .can_attack()
            .build(),
        square: Square::new(file, rank),
        team,
        vision: Vision::default(),
    })
}

fn bishop(team: Team, file: File, rank: Rank) -> PieceBundle {
    PieceBundle {
        kind: PieceKind::SquareCF,
        behavior: Behavior::builder().diagonals().can_attack().build(),
        square: Square::new(file, rank),
        team,
        vision: Vision::default(),
    }
}

fn rook(team: Team, file: File, rank: Rank) -> PieceBundle {
    PieceBundle {
        kind: PieceKind::SquareAH,
        behavior: Behavior::builder().orthogonals().can_attack().build(),
        square: Square::new(file, rank),
        team,
        vision: Vision::default(),
    }
}

fn queen(team: Team, rank: Rank) -> PieceBundle {
    PieceBundle {
        kind: PieceKind::SquareD,
        behavior: Behavior::builder().radials().can_attack().build(),
        square: Square::new(File::D, rank),
        team,
        vision: Vision::default(),
    }
}

fn piece_unicode(icon: PieceIcon, team: Team) -> char {
    match (icon, team) {
        (PieceIcon::King, White) => '\u{2654}',
        (PieceIcon::Queen, White) => '\u{2655}',
        (PieceIcon::Rook, White) => '\u{2656}',
        (PieceIcon::Bishop, White) => '\u{2657}',
        (PieceIcon::Knight, White) => '\u{2658}',
        (PieceIcon::Pawn, White) => '\u{2659}',
        (PieceIcon::King, Black) => '\u{265A}',
        (PieceIcon::Queen, Black) => '\u{265B}',
        (PieceIcon::Rook, Black) => '\u{265C}',
        (PieceIcon::Bishop, Black) => '\u{265D}',
        (PieceIcon::Knight, Black) => '\u{265E}',
        (PieceIcon::Pawn, Black) => '\u{265F}',
    }
}

pub fn classical_board(mut commands: Commands) {
    for team in vec![White, Black] {
        let rank: Rank = if team == White { '1' } else { '8' }.try_into().unwrap();
        let pawn_rank: Rank = if team == White { '2' } else { '7' }.try_into().unwrap();
        // pawns
        for file in 0..=7 {
            commands.spawn((
                pawn(team, file.try_into().unwrap(), pawn_rank),
                PieceIcon::Pawn,
            ));
        }
        // king
        commands.spawn((king(team, rank), PieceIcon::King));
        // queen
        commands.spawn((queen(team, rank), PieceIcon::Queen));
        // rooks
        for file in vec![0, 7].into_iter() {
            commands.spawn((rook(team, file.try_into().unwrap(), rank), PieceIcon::Rook));
        }
        // bishops
        for file in vec![2, 5].into_iter() {
            commands.spawn((
                bishop(team, file.try_into().unwrap(), rank),
                PieceIcon::Bishop,
            ));
        }
        // knights
        for file in vec![1, 6].into_iter() {
            commands.spawn((
                knight(team, file.try_into().unwrap(), rank).unwrap(),
                PieceIcon::Knight,
            ));
        }
    }
}

pub fn classical_ui(
    query: Query<(Entity, &Behavior, &Square, &Team, &Vision, &PieceIcon)>,
    mut writer: EventWriter<MovePieceEvent>,
    mut contexts: EguiContexts,
    mut selected_piece: Local<Option<Entity>>,
) {
    let pieces: HashMap<Square, (Entity, Square, Team, Vision, PieceIcon)> = query
        .iter()
        .map(|(entity, _, square, team, vision, icon)| {
            (
                square.clone(),
                (entity, square.clone(), *team, vision.clone(), *icon),
            )
        })
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
        egui::Grid::new("board_grid").show(ui, |ui| {
            for y in (0..=7).rev() {
                for x in 0..=7 {
                    let square = Square::new(x.try_into().unwrap(), y.try_into().unwrap());

                    let is_target_square = selected_piece_data
                        .map(|(_, _, _, vision, _)| vision.can_target(&square))
                        .unwrap_or_else(|| false);

                    let text = if let Some(piece) = pieces.get(&square) {
                        piece_unicode(piece.4, piece.2)
                    } else if is_target_square {
                        'X'
                    } else {
                        '-'
                    }
                    .to_string();
                    if is_target_square {
                        ui.style_mut().visuals.code_bg_color = Color32::GOLD;
                    }
                    if ui.button(text).clicked() {
                        if let Some((entity, _, _, vision, _)) = selected_piece_data {
                            if vision.can_target(&square) {
                                writer.send(MovePieceEvent(
                                    *entity,
                                    square,
                                    todo!("Implement promotion, or re-use existing board assets"),
                                ));
                                *selected_piece = None;
                            } else if let Some((current_entity, _, _, _, _)) = pieces.get(&square) {
                                *selected_piece = Some(*current_entity);
                            } else {
                                *selected_piece = None;
                            }
                        } else if let Some((current_entity, _, _, _, _)) = pieces.get(&square) {
                            *selected_piece = Some(*current_entity);
                        }
                    };
                }
                ui.end_row();
            }
        });
        ui.label(format!(
            "Selected square: {:?}",
            selected_piece_data.map(|(_, square, _, _, _)| square)
        ));
    });
}
