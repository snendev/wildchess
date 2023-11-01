use bevy::prelude::{
    debug, info, Added, Commands, Entity, EventReader, EventWriter, Name, Query, Res, Time, With,
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

use crate::components::{
    ActionHistory, Clock, ClockConfiguration, Game, GameBoard, HasTurn, History, InGame, Player,
    Ply, WinCondition,
};

use super::{
    events::GameoverEvent, IssueMoveEvent, IssueMutationEvent, RequestMutationEvent, TurnEvent,
};

pub(super) fn spawn_game_entities(
    mut commands: Commands,
    query: Query<(Entity, &GameBoard, Option<&ClockConfiguration>), Added<GameBoard>>,
) {
    for (game_entity, game_board, clock) in query.iter() {
        // add move history to the game
        commands
            .entity(game_entity)
            .insert((Ply::default(), ActionHistory::default()));

        // create an entity to manage board properties
        let board = match game_board {
            GameBoard::Chess
            | GameBoard::WildChess
            | GameBoard::KnightRelayChess
            | GameBoard::SuperRelayChess => Board::chess_board(),
        };
        let board_entity = commands.spawn((board, InGame(game_entity))).id();

        // spawn all game pieces
        let pieces_per_player = match game_board {
            GameBoard::Chess => ClassicalLayout::pieces(),
            GameBoard::WildChess => WildLayout::pieces(),
            GameBoard::KnightRelayChess => KnightRelayLayout::pieces(),
            GameBoard::SuperRelayChess => SuperRelayLayout::pieces(),
        };

        for team in [Team::White, Team::Black].into_iter() {
            let mut player_builder =
                commands.spawn((Player, team, team.orientation(), InGame(game_entity)));
            if team == Team::White {
                player_builder.insert(HasTurn);
            }
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
                    History::<Position>::default(),
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
    game_query: Query<&Ply, With<Game>>,
    board_query: Query<&Board>,
    piece_query: Query<(&Team, &OnBoard, &InGame, Option<&Mutation>)>,
    mut move_reader: EventReader<IssueMoveEvent>,
    mut mutation_reader: EventReader<IssueMutationEvent>,
    mut mutation_request_writer: EventWriter<RequestMutationEvent>,
    mut turn_writer: EventWriter<TurnEvent>,
) {
    for IssueMoveEvent { piece, action } in move_reader.iter() {
        let Ok((team, on_board, in_game, mutation)) = piece_query.get(*piece) else {
            continue;
        };
        let Ok(ply) = game_query.get(in_game.0) else {
            continue;
        };
        if let Some(mutation) = mutation {
            let Ok(board) = board_query.get(on_board.0) else {
                continue;
            };
            match mutation.condition {
                MutationCondition::LocalRank(rank) => {
                    let reoriented_rank = action
                        .landing_square
                        .reorient(team.orientation(), board)
                        .rank;
                    if rank != reoriented_rank {
                        turn_writer.send(TurnEvent::action(
                            *ply,
                            *piece,
                            on_board.0,
                            in_game.0,
                            action.clone(),
                        ));
                    } else if mutation.to_piece.len() == 1 {
                        turn_writer.send(TurnEvent::mutation(
                            *ply,
                            *piece,
                            on_board.0,
                            in_game.0,
                            action.clone(),
                            mutation.to_piece.first().unwrap().clone(),
                        ));
                    } else {
                        mutation_request_writer.send(RequestMutationEvent {
                            piece: *piece,
                            action: action.clone(),
                        });
                    }
                }
                MutationCondition::OnCapture => {
                    unimplemented!("OnCapture promotion not yet implemented");
                }
            }
        } else {
            turn_writer.send(TurnEvent::action(
                *ply,
                *piece,
                on_board.0,
                in_game.0,
                action.clone(),
            ));
        }
    }
    // TODO make a separate system
    for IssueMutationEvent {
        piece,
        action,
        piece_definition,
    } in mutation_reader.iter()
    {
        let Ok((_, on_board, in_game, _)) = piece_query.get(*piece) else {
            continue;
        };
        let Ok(ply) = game_query.get(in_game.0) else {
            continue;
        };
        turn_writer.send(TurnEvent::mutation(
            *ply,
            *piece,
            on_board.0,
            in_game.0,
            action.clone(),
            piece_definition.clone(),
        ));
    }
}

pub(super) fn execute_turn_movement(
    mut commands: Commands,
    mut piece_query: Query<(Entity, &mut Position, &OnBoard)>,
    mut turn_reader: EventReader<TurnEvent>,
) {
    for event in turn_reader.iter() {
        if let Ok((_, mut current_square, _)) = piece_query.get_mut(event.piece) {
            current_square.0 = event.action.landing_square;
        }

        for capture_square in event.action.captures.iter() {
            if let Some(captured_piece) =
                piece_query
                    .iter()
                    .find_map(|(capture_entity, position, board)| {
                        if *position == (*capture_square).into()
                            && capture_entity != event.piece
                            && event.board == board.0
                        {
                            Some(capture_entity)
                        } else {
                            None
                        }
                    })
            {
                // keep the entity around so that we can maintain its position history
                // and visualize it when viewing old ply
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
    mut players_query: Query<(Entity, Option<&mut Clock>, Option<&HasTurn>), With<Player>>,
    mut commands: Commands,
) {
    for (player, clock, my_turn) in players_query.iter_mut() {
        if my_turn.is_some() {
            if let Some(mut clock) = clock {
                clock.pause();
            }
            commands.entity(player).remove::<HasTurn>();
        } else {
            if let Some(mut clock) = clock {
                clock.unpause();
            }
            commands.entity(player).insert(HasTurn);
        }
    }
}

pub(super) fn track_turn_history(
    mut game_query: Query<(&mut Ply, &mut ActionHistory), With<Game>>,
    mut turn_reader: EventReader<TurnEvent>,
) {
    for TurnEvent {
        ply,
        piece,
        game,
        action,
        ..
    } in turn_reader.iter()
    {
        let Ok((mut game_ply, mut history)) = game_query.get_mut(*game) else {
            continue;
        };
        // TODO: in a future with 4-player, does this lead to bugs?
        if *game_ply != *ply {
            debug!(
                "Turn ply {:?} does not match current game ply {:?}",
                *ply, *game_ply
            );
            continue;
        }
        history.push(*piece, action.clone());
        game_ply.increment();
    }
}

pub(super) fn last_action(mut reader: EventReader<TurnEvent>) -> Option<Action> {
    reader.iter().last().map(|event| event.action.clone())
}

pub(super) fn detect_gameover(
    game_query: Query<(Entity, &WinCondition)>,
    royal_query: Query<(&InGame, &Team, Option<&Position>), With<Royal>>,
    mut gameover_writer: EventWriter<GameoverEvent>,
) {
    for (game_entity, win_condition) in game_query.iter() {
        match win_condition {
            WinCondition::RoyalCaptureAll => {
                let all_captured = |current_team: Team| {
                    royal_query
                        .iter()
                        .filter(|(in_game, team, position)| {
                            in_game.0 == game_entity && **team == current_team && position.is_some()
                        })
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
                        .filter(|(in_game, team, position)| {
                            in_game.0 == game_entity && **team == current_team && position.is_none()
                        })
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
