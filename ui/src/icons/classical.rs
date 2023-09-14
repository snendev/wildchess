use bevy::prelude::{Added, Changed, Commands, Entity, Or, Query};

use chess_gameplay::{pieces::PatternBehavior, team::Team};

use super::PieceIcon;

fn piece_unicode(piece: PieceIdentity, team: Team) -> char {
    match (piece, team) {
        // king
        (PieceIdentity::King, White) => '\u{2654}',
        (PieceIdentity::King, Black) => '\u{265A}',
        // queen
        (PieceIdentity::Queen, White) => '\u{2655}',
        (PieceIdentity::Queen, Black) => '\u{265B}',
        // rook
        (PieceIdentity::Rook, White) => '\u{2656}',
        (PieceIdentity::Rook, Black) => '\u{265C}',
        // bishop
        (PieceIdentity::Bishop, White) => '\u{2657}',
        (PieceIdentity::Bishop, Black) => '\u{265D}',
        // knight
        (PieceIdentity::Knight, White) => '\u{2658}',
        (PieceIdentity::Knight, Black) => '\u{265E}',
        // pawn
        (PieceIdentity::Pawn, White) => '\u{2659}',
        (PieceIdentity::Pawn, Black) => '\u{265F}',
    }
}

fn override_icons(
    mut commands: Commands,
    mut query: Query<
        (Entity, &PatternBehavior, &Team, Option<&mut PieceIcon>),
        Or<(Changed<PatternBehavior>, Added<PieceIcon>)>,
    >,
) {
    for (entity, behavior, team, icon) in query.iter_mut() {
        let identity = if *behavior == king() {
            PieceIdentity::King
        } else if *behavior == queen() {
            PieceIdentity::Queen
        } else if *behavior == rook() {
            PieceIdentity::Rook
        } else if *behavior == bishop() {
            PieceIdentity::Bishop
        } else if *behavior == knight() {
            PieceIdentity::Knight
        } else if *behavior == pawn() {
            PieceIdentity::Pawn
        } else {
            panic!("Only use classical piece behaviors here.");
        };
        let piece_icon = piece_unicode(identity, *team);
        if let Some(mut icon) = icon {
            *icon = PieceIcon::Character(piece_icon);
        } else {
            commands
                .entity(entity)
                .insert(PieceIcon::Character(piece_icon));
        }
    }
}
