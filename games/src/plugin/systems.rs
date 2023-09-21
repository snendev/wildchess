use bevy::prelude::{Added, Commands, Entity, EventReader, EventWriter, Query, Res, Time, With};

use chess::{
    behavior::PieceBehaviorsBundle,
    board::Board,
    pieces::{Action, Actions, Mutation, MutationCondition, Position, Royal},
    team::Team,
};
use layouts::{ClassicalLayout, KnightRelayLayout, SuperRelayLayout, WildLayout};

use crate::{
    components::{Clock, GameBoard, Player, PlayerBundle, Turn, WinCondition},
    events::{IssueMoveEvent, RequestMutationEvent},
    IssueMutationEvent, TurnEvent,
};

pub(super) fn detect_turn(
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

pub(super) fn execute_turn_movement(
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

pub(super) fn execute_turn_mutations(
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

pub(super) fn end_turn(
    mut players_query: Query<(Entity, &mut Clock, Option<&Turn>), With<Player>>,
    mut commands: Commands,
) {
    for (player, mut clock, my_turn) in players_query.iter_mut() {
        if my_turn.is_some() {
            clock.pause();
            commands.entity(player).remove::<Turn>();
        } else {
            clock.unpause();
            commands.entity(player).insert(Turn);
        }
    }
}

pub(super) fn clear_actions(mut piece_query: Query<&mut Actions>) {
    for mut actions in piece_query.iter_mut() {
        actions.clear();
    }
}

pub(super) fn last_action(mut reader: EventReader<TurnEvent>) -> Option<Action> {
    reader.iter().last().map(|event| event.action.clone())
}

pub(super) fn detect_gameover(
    game_query: Query<Entity, &WinCondition>,
    royal_query: Query<(&Position, &Team), With<Royal>>,
) {
}

pub(super) fn spawn_game_entities(
    mut commands: Commands,
    query: Query<(Entity, &GameBoard), Added<GameBoard>>,
) {
    for (entity, game_board) in query.iter() {
        let board = match game_board {
            GameBoard::Chess
            | GameBoard::WildChess
            | GameBoard::KnightRelayChess
            | GameBoard::SuperRelayChess => Board::chess_board(),
        };
        commands.entity(entity).insert(board);
        commands.spawn((PlayerBundle::new(Team::White), Turn));
        commands.spawn((PlayerBundle::new(Team::Black)));
        match game_board {
            GameBoard::Chess => ClassicalLayout::spawn_pieces(&mut commands),
            GameBoard::WildChess => WildLayout::spawn_pieces(&mut commands),
            GameBoard::KnightRelayChess => KnightRelayLayout::spawn_pieces(&mut commands),
            GameBoard::SuperRelayChess => SuperRelayLayout::spawn_pieces(&mut commands),
        }
    }
}

pub(super) fn tick_clocks(mut query: Query<&mut Clock>, time: Res<Time>) {
    for mut clock in query.iter_mut() {
        clock.tick(time.delta());
    }
}
