use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, With};

use chess::{
    behavior::PieceBehaviorsBundle,
    pieces::{Action, Actions, Mutation, MutationCondition, Position, Royal},
};

use crate::{
    components::{Player, Turn},
    events::{IssueMoveEvent, RequestMutationEvent},
    IssueMutationEvent, TurnEvent,
};

pub(crate) fn detect_turn(
    piece_query: Query<&Mutation>,
    mut move_reader: EventReader<IssueMoveEvent>,
    mut mutation_reader: EventReader<IssueMutationEvent>,
    mut mutation_request_writer: EventWriter<RequestMutationEvent>,
    mut turn_writer: EventWriter<TurnEvent>,
) {
    for IssueMoveEvent(entity, action) in move_reader.iter() {
        if let Ok(mutation) = piece_query.get(*entity) {
            match mutation.condition {
                MutationCondition::Rank(rank) => {
                    if rank != action.landing_square.rank {
                        turn_writer.send(TurnEvent::action(*entity, action.clone()));
                    } else if mutation.to_piece.len() == 1 {
                        turn_writer.send(TurnEvent::mutation(
                            *entity,
                            action.clone(),
                            mutation.to_piece.first().unwrap().clone(),
                        ));
                    } else {
                        mutation_request_writer.send(RequestMutationEvent(*entity, action.clone()));
                    }
                }
                MutationCondition::OnCapture => {
                    unimplemented!("OnCapture promotion not yet implemented");
                }
            }
        } else {
            turn_writer.send(TurnEvent::action(*entity, action.clone()));
        }
    }
    for IssueMutationEvent(entity, action, piece) in mutation_reader.iter() {
        turn_writer.send(TurnEvent::mutation(*entity, action.clone(), piece.clone()));
    }
}

pub(crate) fn execute_turn_movement(
    mut commands: Commands,
    mut piece_query: Query<(Entity, &mut Position)>,
    mut turn_reader: EventReader<TurnEvent>,
) {
    for event in turn_reader.iter() {
        if let Ok((_, mut current_square)) = piece_query.get_mut(event.entity) {
            current_square.0 = event.action.landing_square;
        }

        for capture_square in event.action.captures.iter() {
            if let Some(captured_piece) =
                piece_query.iter().find_map(|(capture_entity, position)| {
                    if *position == (*capture_square).into() && capture_entity != event.entity {
                        Some(capture_entity)
                    } else {
                        None
                    }
                })
            {
                // TODO: should this despawn, or is there a good reason to keep the entity around?
                commands.entity(captured_piece).remove::<Position>();
            }
        }
    }
}

pub(crate) fn execute_turn_mutations(
    mut commands: Commands,
    mut turn_reader: EventReader<TurnEvent>,
) {
    for event in turn_reader.iter() {
        if let Some(mutated_piece) = &event.mutation {
            // remove any existing behaviors and mutation
            commands
                .entity(event.entity)
                .remove::<PieceBehaviorsBundle>();
            commands.entity(event.entity).remove::<Mutation>();
            commands.entity(event.entity).remove::<Royal>();

            // add subsequent mutation if specified
            if let Some(new_mutation) = &mutated_piece.mutation {
                commands.entity(event.entity).insert(new_mutation.clone());
            }

            // add Royal if specified
            if mutated_piece.royal.is_some() {
                commands.entity(event.entity).insert(Royal);
            }

            // add all specified behaviors
            if let Some(mutation_behavior) = &mutated_piece.behaviors.pattern {
                commands
                    .entity(event.entity)
                    .insert(mutation_behavior.clone());
            }

            if let Some(mutation_behavior) = &mutated_piece.behaviors.en_passant {
                commands
                    .entity(event.entity)
                    .insert(mutation_behavior.clone());
            }

            if let Some(mutation_behavior) = &mutated_piece.behaviors.mimic {
                commands
                    .entity(event.entity)
                    .insert(mutation_behavior.clone());
            }

            if let Some(mutation_behavior) = &mutated_piece.behaviors.relay {
                commands
                    .entity(event.entity)
                    .insert(mutation_behavior.clone());
            }
        }
    }
}

pub(crate) fn end_turn(
    players_query: Query<(Entity, Option<&Turn>), With<Player>>,
    mut commands: Commands,
) {
    for (player, my_turn) in players_query.iter() {
        if my_turn.is_some() {
            commands.entity(player).remove::<Turn>();
        } else {
            commands.entity(player).insert(Turn);
        }
    }
}

pub(crate) fn clear_actions(mut piece_query: Query<&mut Actions>) {
    for mut actions in piece_query.iter_mut() {
        actions.clear();
    }
}

pub(crate) fn last_action(mut reader: EventReader<TurnEvent>) -> Option<Action> {
    reader.iter().last().map(|event| event.action.clone())
}
