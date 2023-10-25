use bevy::prelude::{
    info, Added, Commands, Entity, EventReader, EventWriter, Name, Query, Res, Time, With,
};

use chess::{
    actions::Action,
    behavior::PieceBehaviorsBundle,
    board::{Board, OnBoard},
    pieces::{Mutation, MutationCondition, PieceBundle, Position, Royal},
    team::Team,
};
use layouts::{
    ClassicalLayout, KnightRelayLayout, PieceSpecification, SuperRelayLayout, WildLayout,
};

use crate::components::{Clock, ClockConfiguration, GameBoard, InGame, Player, Turn, WinCondition};

use super::{
    events::GameoverEvent, IssueMoveEvent, IssueMutationEvent, RequestMutationEvent, TurnEvent,
};

pub(super) fn spawn_game_entities(
    mut commands: Commands,
    query: Query<(Entity, &GameBoard, Option<&ClockConfiguration>), Added<GameBoard>>,
) {
    for (game_entity, game_board, clock) in query.iter() {
        let board = match game_board {
            GameBoard::Chess
            | GameBoard::WildChess
            | GameBoard::KnightRelayChess
            | GameBoard::SuperRelayChess => Board::chess_board(),
        };
        let board_entity = commands.spawn((board, InGame(game_entity))).id();

        let pieces_per_player = match game_board {
            GameBoard::Chess => ClassicalLayout::pieces(),
            GameBoard::WildChess => WildLayout::pieces(),
            GameBoard::KnightRelayChess => KnightRelayLayout::pieces(),
            GameBoard::SuperRelayChess => SuperRelayLayout::pieces(),
        };

        for team in [Team::White, Team::Black].into_iter() {
            let mut player_builder =
                commands.spawn((Player, team, team.orientation(), InGame(game_entity), Turn));
            if let Some(ClockConfiguration { clock }) = clock {
                player_builder.insert(clock.clone());
            }

            for PieceSpecification {
                piece,
                start_square,
            } in pieces_per_player.iter()
            {
                let start_square = start_square.reorient(team.orientation(), &board);
                let name = Name::new(format!("{:?} {}-{:?}", team, start_square, piece.identity));

                let mut piece_builder = commands.spawn((
                    name,
                    piece.identity,
                    PieceBundle::new(start_square.into(), team),
                    InGame(game_entity),
                    OnBoard(board_entity),
                ));

                if piece.royal.is_some() {
                    piece_builder.insert(Royal);
                }
                if let Some(mutation) = &piece.mutation {
                    piece_builder.insert(mutation.clone());
                }
                if let Some(behavior) = piece.behaviors.en_passant {
                    piece_builder.insert(behavior);
                }
                if let Some(behavior) = piece.behaviors.mimic {
                    piece_builder.insert(behavior);
                }
                if let Some(behavior) = &piece.behaviors.pattern {
                    piece_builder.insert(behavior.clone());
                }
                if let Some(behavior) = &piece.behaviors.relay {
                    piece_builder.insert(behavior.clone());
                }
            }
        }
    }
}

pub(super) fn detect_turn(
    board_query: Query<&Board>,
    piece_query: Query<(&Team, &Mutation, &OnBoard)>,
    mut move_reader: EventReader<IssueMoveEvent>,
    mut mutation_reader: EventReader<IssueMutationEvent>,
    mut mutation_request_writer: EventWriter<RequestMutationEvent>,
    mut turn_writer: EventWriter<TurnEvent>,
) {
    for IssueMoveEvent {
        piece,
        game,
        action,
    } in move_reader.iter()
    {
        if let Ok((team, mutation, on_board)) = piece_query.get(*piece) {
            match mutation.condition {
                MutationCondition::LocalRank(rank) => {
                    let Ok(board) = board_query.get(on_board.0) else {
                        continue;
                    };
                    let reoriented_rank = action
                        .landing_square
                        .reorient(team.orientation(), board)
                        .rank;
                    if rank != reoriented_rank {
                        turn_writer.send(TurnEvent::action(*piece, *game, action.clone()));
                    } else if mutation.to_piece.len() == 1 {
                        turn_writer.send(TurnEvent::mutation(
                            *piece,
                            *game,
                            action.clone(),
                            mutation.to_piece.first().unwrap().clone(),
                        ));
                    } else {
                        mutation_request_writer.send(RequestMutationEvent {
                            piece: *piece,
                            game: *game,
                            action: action.clone(),
                        });
                    }
                }
                MutationCondition::OnCapture => {
                    unimplemented!("OnCapture promotion not yet implemented");
                }
            }
        } else {
            turn_writer.send(TurnEvent::action(*piece, *game, action.clone()));
        }
    }
    for IssueMutationEvent {
        piece,
        game,
        action,
        piece_definition,
    } in mutation_reader.iter()
    {
        turn_writer.send(TurnEvent::mutation(
            *piece,
            *game,
            action.clone(),
            piece_definition.clone(),
        ));
    }
}

pub(super) fn execute_turn_movement(
    mut commands: Commands,
    mut piece_query: Query<(Entity, &mut Position)>,
    mut turn_reader: EventReader<TurnEvent>,
) {
    for event in turn_reader.iter() {
        if let Ok((_, mut current_square)) = piece_query.get_mut(event.piece) {
            current_square.0 = event.action.landing_square;
        }

        for capture_square in event.action.captures.iter() {
            if let Some(captured_piece) =
                piece_query.iter().find_map(|(capture_entity, position)| {
                    if *position == (*capture_square).into() && capture_entity != event.piece {
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
                .entity(event.piece)
                .remove::<PieceBehaviorsBundle>();
            commands.entity(event.piece).remove::<Mutation>();
            commands.entity(event.piece).remove::<Royal>();

            // add subsequent mutation if specified
            if let Some(new_mutation) = &mutated_piece.mutation {
                commands.entity(event.piece).insert(new_mutation.clone());
            }

            // add Royal if specified
            if mutated_piece.royal.is_some() {
                commands.entity(event.piece).insert(Royal);
            }

            // add all specified behaviors
            if let Some(mutation_behavior) = &mutated_piece.behaviors.pattern {
                commands
                    .entity(event.piece)
                    .insert(mutation_behavior.clone());
            }

            if let Some(mutation_behavior) = &mutated_piece.behaviors.en_passant {
                commands.entity(event.piece).insert(*mutation_behavior);
            }

            if let Some(mutation_behavior) = &mutated_piece.behaviors.mimic {
                commands.entity(event.piece).insert(*mutation_behavior);
            }

            if let Some(mutation_behavior) = &mutated_piece.behaviors.relay {
                commands
                    .entity(event.piece)
                    .insert(mutation_behavior.clone());
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn end_turn(
    mut players_query: Query<(Entity, Option<&mut Clock>, Option<&Turn>), With<Player>>,
    mut commands: Commands,
) {
    for (player, clock, my_turn) in players_query.iter_mut() {
        if my_turn.is_some() {
            if let Some(mut clock) = clock {
                clock.pause();
            }
            commands.entity(player).remove::<Turn>();
        } else {
            if let Some(mut clock) = clock {
                clock.unpause();
            }
            commands.entity(player).insert(Turn);
        }
    }
}

pub(super) fn last_action(mut reader: EventReader<TurnEvent>) -> Option<Action> {
    reader.iter().last().map(|event| event.action.clone())
}

pub(super) fn detect_gameover(
    game_query: Query<(Entity, &WinCondition)>,
    royal_query: Query<(&Team, Option<&Position>), With<Royal>>,
    mut gameover_writer: EventWriter<GameoverEvent>,
) {
    // TODO: enable running multiple boards
    let Ok((_game_entity, win_condition)) = game_query.get_single() else {
        return;
    };

    match win_condition {
        WinCondition::RoyalCaptureAll => {
            let all_captured = |current_team: Team| {
                royal_query
                    .iter()
                    .filter(|(team, position)| **team == current_team && position.is_some())
                    .count()
                    == 0
            };
            if all_captured(Team::White) {
                gameover_writer.send(GameoverEvent {
                    winner: Team::Black,
                })
            }
            if all_captured(Team::Black) {
                gameover_writer.send(GameoverEvent {
                    winner: Team::White,
                })
            }
        }
        WinCondition::RoyalCapture => {
            let any_captured = |current_team: Team| {
                royal_query
                    .iter()
                    .filter(|(team, position)| **team == current_team && position.is_none())
                    .count()
                    > 0
            };
            if any_captured(Team::White) {
                gameover_writer.send(GameoverEvent {
                    winner: Team::Black,
                })
            }
            if any_captured(Team::Black) {
                gameover_writer.send(GameoverEvent {
                    winner: Team::White,
                })
            }
        }
        WinCondition::RaceToRank(_rank) => {
            unimplemented!("TODO: Implement Racing Kings!")
        }
        WinCondition::RaceToRegion(_goal_squares) => {
            unimplemented!("TODO: Implement Racing Kings!")
        }
    }
}

pub fn log_gameover_events(mut gameovers: EventReader<GameoverEvent>) {
    for gameover in gameovers.iter() {
        info!("Team {:?} won!", gameover.winner);
        // TODO: display this somewhere
    }
}

pub(super) fn tick_clocks(mut query: Query<&mut Clock>, time: Res<Time>) {
    for mut clock in query.iter_mut() {
        clock.tick(time.delta());
    }
}
