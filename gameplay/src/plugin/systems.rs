use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, With};

use chess::pieces::{Action, Actions, Mutation, MutationCondition, PatternBehavior, Position};

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
                        turn_writer.send(TurnEvent::Action(*entity, action.clone()));
                    } else if mutation.options.len() == 1 {
                        turn_writer.send(TurnEvent::Mutation(
                            *entity,
                            action.clone(),
                            mutation.options.first().unwrap().clone(),
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
            turn_writer.send(TurnEvent::Action(*entity, action.clone()));
        }
    }
    for IssueMutationEvent(entity, action, piece) in mutation_reader.iter() {
        turn_writer.send(TurnEvent::Mutation(*entity, action.clone(), piece.clone()));
    }
}

pub(crate) fn execute_turn(
    mut commands: Commands,
    mut piece_query: Query<(Entity, &mut Position, &mut PatternBehavior)>,
    mut turn_reader: EventReader<TurnEvent>,
) {
    for event in turn_reader.iter() {
        let (entity, action) = match event {
            TurnEvent::Action(entity, action) => (*entity, action),
            TurnEvent::Mutation(entity, action, _) => (*entity, action),
        };

        if let TurnEvent::Mutation(_, _, mutated_piece) = event {
            if let Ok((_, _, mut behavior)) = piece_query.get_mut(entity) {
                *behavior = mutated_piece.behavior.clone();
                commands.entity(entity).remove::<Mutation>();
            }
        }

        if let Ok((_, mut current_square, _)) = piece_query.get_mut(entity) {
            current_square.0 = action.landing_square;
        }

        for capture_square in action.captures.iter() {
            if let Some(captured_piece) =
                piece_query
                    .iter()
                    .find_map(|(capture_entity, position, _)| {
                        if *position == (*capture_square).into() && capture_entity != entity {
                            Some(entity)
                        } else {
                            None
                        }
                    })
            {
                commands.entity(captured_piece).remove::<Position>();
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
    let mut last_action = None;
    for event in reader.iter() {
        last_action = Some(match event {
            TurnEvent::Action(_, action) => action.clone(),
            TurnEvent::Mutation(_, action, _) => action.clone(),
        });
    }
    last_action
}
