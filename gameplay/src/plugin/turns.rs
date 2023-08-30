use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, With};

use chess::pieces::{Behavior, Position, Promotable};

use crate::{
    components::{Player, Turn},
    events::{IssueMoveEvent, RequestPromotionEvent},
    IssuePromotionEvent, TurnEvent,
};

pub fn detect_turn(
    piece_query: Query<&Promotable>,
    mut move_reader: EventReader<IssueMoveEvent>,
    mut promotion_reader: EventReader<IssuePromotionEvent>,
    mut promotion_request_writer: EventWriter<RequestPromotionEvent>,
    mut turn_writer: EventWriter<TurnEvent>,
) {
    for IssueMoveEvent(movement) in move_reader.iter() {
        if let Ok(promotable) = piece_query.get(movement.entity) {
            if !promotable.ranks.contains(&movement.target_square.rank) {
                turn_writer.send(TurnEvent::Movement(movement.clone()));
            } else if promotable.behaviors.len() == 1 {
                let promotion_behavior = promotable.behaviors.first().unwrap().clone();
                turn_writer.send(TurnEvent::Promotion(movement.clone(), promotion_behavior));
            } else {
                promotion_request_writer.send(RequestPromotionEvent(movement.clone()));
            }
        } else {
            turn_writer.send(TurnEvent::Movement(movement.clone()));
        }
    }
    for IssuePromotionEvent(movement, behavior) in promotion_reader.iter() {
        turn_writer.send(TurnEvent::Promotion(movement.clone(), behavior.clone()));
    }
}

pub fn execute_turn(
    mut commands: Commands,
    mut piece_query: Query<(&mut Position, &mut Behavior)>,
    mut turn_reader: EventReader<TurnEvent>,
) {
    for event in turn_reader.iter() {
        let movement = match event {
            TurnEvent::Movement(movement) => movement,
            TurnEvent::Promotion(movement, _) => movement,
        };

        if let TurnEvent::Promotion(movement, upgraded_behavior) = event {
            if let Ok((_, mut behavior)) = piece_query.get_mut(movement.entity) {
                *behavior = upgraded_behavior.clone();
                commands.entity(movement.entity).remove::<Promotable>();
            }
        }

        if let Ok((mut current_square, _)) = piece_query.get_mut(movement.entity) {
            current_square.0 = movement.target_square;
        }
    }
}

pub fn end_turn(
    players_query: Query<(Entity, Option<&Turn>), With<Player>>,
    mut turn_reader: EventReader<TurnEvent>,
    mut commands: Commands,
) {
    if !turn_reader.is_empty() {
        for (player, my_turn) in players_query.iter() {
            if my_turn.is_some() {
                commands.entity(player).remove::<Turn>();
            } else {
                commands.entity(player).insert(Turn);
            }
        }
    }
    turn_reader.clear();
}
