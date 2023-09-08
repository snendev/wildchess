use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, With};

use chess::pieces::{Behavior, Mutation, MutationCondition, Position};

use crate::{
    components::{Player, Turn},
    events::{IssueMoveEvent, RequestMutationEvent},
    IssueMutationEvent, TurnEvent,
};

pub fn detect_turn(
    piece_query: Query<&Mutation>,
    mut move_reader: EventReader<IssueMoveEvent>,
    mut mutation_reader: EventReader<IssueMutationEvent>,
    mut mutation_request_writer: EventWriter<RequestMutationEvent>,
    mut turn_writer: EventWriter<TurnEvent>,
) {
    for IssueMoveEvent(movement) in move_reader.iter() {
        if let Ok(mutation) = piece_query.get(movement.entity) {
            match mutation.condition {
                MutationCondition::Rank(rank) => {
                    if rank == movement.target_square.rank {
                        turn_writer.send(TurnEvent::Movement(movement.clone()));
                    } else if mutation.options.len() == 1 {
                        turn_writer.send(TurnEvent::Mutation(
                            movement.clone(),
                            mutation.options.first().unwrap().clone(),
                        ));
                    } else {
                        mutation_request_writer.send(RequestMutationEvent(movement.clone()));
                    }
                }
            }
        } else {
            turn_writer.send(TurnEvent::Movement(movement.clone()));
        }
    }
    for IssueMutationEvent(movement, behavior) in mutation_reader.iter() {
        turn_writer.send(TurnEvent::Mutation(movement.clone(), behavior.clone()));
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
            TurnEvent::Mutation(movement, _) => movement,
        };

        if let TurnEvent::Mutation(movement, mutation_option) = event {
            if let Ok((_, mut behavior)) = piece_query.get_mut(movement.entity) {
                *behavior = mutation_option.behavior.clone();
                commands.entity(movement.entity).remove::<Mutation>();
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
