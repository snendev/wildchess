use bevy::prelude::{info, Commands, DetectChanges, Entity, Query, Ref};

use chess::pieces::Position;

pub fn capture_pieces(mut commands: Commands, piece_query: Query<(Entity, Ref<Position>)>) {
    for (piece, position_ref) in piece_query.iter() {
        if position_ref.is_changed() {
            let captured_piece = piece_query.iter().find_map(|(entity, position)| {
                if *position == *position_ref && piece != entity {
                    Some(entity)
                } else {
                    None
                }
            });
            if let Some(captured_piece) = captured_piece {
                info!("???");
                commands.entity(captured_piece).remove::<Position>();
            }
        }
    }
}
