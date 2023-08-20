use bevy::prelude::{Commands, EventReader, EventWriter, Query, With};

use crate::{
    components::{Behavior, Position, Promotable, Team},
    Movement, PieceEvent, Promotion, RequestPromotion,
};

pub fn move_pieces(
    mut piece_query: Query<(&mut Position, &Team, Option<&Promotable>)>,
    mut move_reader: EventReader<PieceEvent<Movement>>,
    mut promotion_request_writer: EventWriter<PieceEvent<RequestPromotion>>,
    mut promotion_writer: EventWriter<PieceEvent<Promotion>>,
) {
    for event in move_reader.iter() {
        let movement = event.get();
        if let Ok((mut current_square, team, promotable)) = piece_query.get_mut(movement.entity) {
            current_square.0 = movement.target_square;
            if let Some(promotable) = promotable {
                if promotable.local_rank.from_local(team) != movement.target_square.rank {
                    continue;
                }
                if promotable.behaviors.len() == 1 {
                    let promotion_behavior = promotable.behaviors.first().unwrap().clone();
                    promotion_writer.send(event.to_promotion(promotion_behavior));
                } else {
                    promotion_request_writer.send(PieceEvent::<RequestPromotion>::from(event))
                }
            }
        }
    }
}

pub fn promote_pieces(
    mut commands: Commands,
    mut promotion_reader: EventReader<PieceEvent<Promotion>>,
    mut piece_query: Query<&mut Behavior, With<Promotable>>,
) {
    for event in promotion_reader.iter() {
        let Promotion {
            entity,
            behavior: upgraded_behavior,
        } = event.get();
        if let Ok(mut behavior) = piece_query.get_mut(*entity) {
            *behavior = upgraded_behavior.clone();
            commands.entity(*entity).remove::<Promotable>();
        }
    }
}
