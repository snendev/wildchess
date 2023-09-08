use bevy::prelude::{
    Added, Changed, Commands, Entity, EventWriter, Or, PostUpdate, Query, Startup,
};

use bevy_geppetto::Test;
use chess_boards::{classical::ClassicalIdentity, BoardPlugin, BuildClassicalBoardEvent};
use chess_gameplay::{
    chess::team::Team::{self, Black, White},
    GameplayPlugin,
};

use wildchess_ui::{EguiBoardUIPlugin, PieceIcon};

fn main() {
    Test {
        label: "test classical board".to_string(),
        setup: |app| {
            app.add_plugins((GameplayPlugin, EguiBoardUIPlugin, BoardPlugin))
                .add_systems(Startup, spawn_board)
                .add_systems(PostUpdate, override_icons);
        },
    }
    .run()
}

fn spawn_board(mut writer: EventWriter<BuildClassicalBoardEvent>) {
    writer.send(BuildClassicalBoardEvent);
}

fn piece_unicode(piece: ClassicalIdentity, team: Team) -> char {
    match (piece, team) {
        // king
        (ClassicalIdentity::King, White) => '\u{2654}',
        (ClassicalIdentity::King, Black) => '\u{265A}',
        // queen
        (ClassicalIdentity::Queen, White) => '\u{2655}',
        (ClassicalIdentity::Queen, Black) => '\u{265B}',
        // rook
        (ClassicalIdentity::Rook, White) => '\u{2656}',
        (ClassicalIdentity::Rook, Black) => '\u{265C}',
        // bishop
        (ClassicalIdentity::Bishop, White) => '\u{2657}',
        (ClassicalIdentity::Bishop, Black) => '\u{265D}',
        // knight
        (ClassicalIdentity::Knight, White) => '\u{2658}',
        (ClassicalIdentity::Knight, Black) => '\u{265E}',
        // pawn
        (ClassicalIdentity::Pawn, White) => '\u{2659}',
        (ClassicalIdentity::Pawn, Black) => '\u{265F}',
    }
}

fn override_icons(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ClassicalIdentity, &Team, Option<&mut PieceIcon>),
        Or<(Changed<ClassicalIdentity>, Added<PieceIcon>)>,
    >,
) {
    for (entity, identity, team, icon) in query.iter_mut() {
        let piece_icon = piece_unicode(*identity, *team);
        if let Some(mut icon) = icon {
            *icon = PieceIcon::Character(piece_icon);
        } else {
            commands
                .entity(entity)
                .insert(PieceIcon::Character(piece_icon));
        }
    }
}
