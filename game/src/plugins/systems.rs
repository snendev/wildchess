use bevy::{
    prelude::{Changed, Commands, DetectChanges, Entity, EventReader, Query, Ref, Res},
    utils::HashMap,
};

use crate::{
    board::{GamePieces, Pawn},
    Behavior, MovePieceEvent, Square, Team, Vision,
};

pub fn calculate_piece_vision(
    mut piece_query: Query<(Entity, &Behavior, &Square, &Team, &mut Vision)>,
    update_query: Query<Entity, Changed<Square>>,
) {
    if !update_query.is_empty() {
        let pieces: HashMap<Square, (Entity, Team)> = piece_query
            .iter()
            .map(|(entity, _, square, team, _)| (square.clone(), (entity, *team)))
            .collect();

        for (_, behavior, square, team, mut vision) in piece_query.iter_mut() {
            vision.set(behavior.search(square, *team, &pieces));
        }
    }
}

pub fn capture_pieces(mut commands: Commands, piece_query: Query<(Entity, Ref<Square>)>) {
    for (piece, square_ref) in piece_query.iter() {
        if square_ref.is_changed() {
            let captured_piece = piece_query.iter().find_map(|(entity, square)| {
                if *square == *square_ref && piece != entity {
                    Some(entity)
                } else {
                    None
                }
            });
            if let Some(captured_piece) = captured_piece {
                commands.entity(captured_piece).remove::<Square>();
            }
        }
    }
}

pub fn move_pieces(
    mut commands: Commands,
    mut piece_query: Query<(&mut Square, &mut Behavior)>,
    mut reader: EventReader<MovePieceEvent>,
    game_pieces: Res<GamePieces>,
) {
    for MovePieceEvent(entity, square, promotion) in reader.iter() {
        if let Ok((mut current_square, mut behavior)) = piece_query.get_mut(*entity) {
            *current_square = square.clone();
            if let Some(promotion) = promotion {
                *behavior = game_pieces
                    .0
                    .iter()
                    .find_map(|(id, piece_behavior)| {
                        if *id == promotion.to_piece {
                            Some(piece_behavior)
                        } else {
                            None
                        }
                    })
                    .unwrap()
                    .clone();
                commands
                    .entity(*entity)
                    .remove::<Pawn>()
                    .insert(promotion.to_piece);
            }
        }
    }
}
