use bevy_core::Name;
use bevy_ecs::{
    prelude::{Commands, Entity, EventReader, EventWriter, Query, With},
    query::Added,
};

use bevy_replicon::{
    core::Replicated,
    prelude::{FromClient, SendMode, ToClients},
};

use chess::{
    actions::LastAction,
    behavior::{BoardPieceCache, BoardThreatsCache, PieceBehaviorsBundle},
    board::{Board, OnBoard},
    pieces::{Mutation, MutationCondition, PieceBundle, PieceIdentity, Position, Royal},
    team::Team,
};
use layouts::PieceSpecification;
use replication::Client;

use crate::{
    components::{
        ActionHistory, Clock, Game, GameBoard, HasTurn, History, InGame, IsActiveGame, PieceSet,
        Ply, WinCondition,
    },
    gameplay::components::GameOver,
};

use super::{RequestTurnEvent, RequireMutationEvent, TurnEvent};

pub(super) fn detect_turn(
    game_query: Query<&Ply, IsActiveGame>,
    board_query: Query<&Board>,
    player_query: Query<(&Team, &Client), With<HasTurn>>,
    piece_query: Query<(&Team, &OnBoard, &InGame, Option<&Mutation>)>,
    mut requested_turns: EventReader<FromClient<RequestTurnEvent>>,
    mut require_mutation_writer: EventWriter<ToClients<RequireMutationEvent>>,
    mut turn_writer: EventWriter<TurnEvent>,
) {
    for FromClient {
        event:
            RequestTurnEvent {
                piece,
                action,
                promotion,
            },
        client_id,
    } in requested_turns.read()
    {
        let Some((player_team, player)) = player_query
            .iter()
            .find(|(_, player)| player.id == *client_id)
        else {
            continue;
        };
        let Ok((team, on_board, in_game, mutation)) = piece_query.get(*piece) else {
            continue;
        };
        if team != player_team {
            continue;
        }
        let Ok(ply) = game_query.get(in_game.0) else {
            continue;
        };

        if let Some(mutation) = mutation {
            let Ok(board) = board_query.get(on_board.0) else {
                continue;
            };
            match mutation.condition {
                MutationCondition::LocalRank(rank) => {
                    let reoriented_rank =
                        action.movement.to.reorient(team.orientation(), board).rank;
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
                    } else if let Some(promotion) = promotion {
                        turn_writer.send(TurnEvent::mutation(
                            *ply,
                            *piece,
                            on_board.0,
                            in_game.0,
                            action.clone(),
                            promotion.clone(),
                        ));
                    } else {
                        require_mutation_writer.send(ToClients {
                            mode: SendMode::Direct(player.id),
                            event: RequireMutationEvent {
                                piece: *piece,
                                action: action.clone(),
                            },
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
}

pub(super) fn execute_turn_movement(
    mut commands: Commands,
    mut piece_query: Query<(Entity, &mut Position, &OnBoard)>,
    mut turn_reader: EventReader<TurnEvent>,
) {
    for event in turn_reader.read() {
        if let Ok((_, mut current_square, _)) = piece_query.get_mut(event.piece) {
            current_square.0 = event.action.movement.to;
        }

        for (entity, additional_movement) in event.action.side_effects.iter() {
            if let Ok((_, mut current_square, _)) = piece_query.get_mut(*entity) {
                current_square.0 = additional_movement.to;
            }
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
                // #[cfg(feature = "log")]
                // bevy_log::info!("Piece {}", );

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
    query: Query<&Position>,
) {
    for event in turn_reader.read() {
        if let Some(mutated_piece) = &event.mutation {
            // remove any existing behaviors and mutation
            commands
                .entity(event.piece)
                .remove::<PieceBehaviorsBundle>();
            commands.entity(event.piece).remove::<Mutation>();
            commands.entity(event.piece).remove::<PieceIdentity>();
            commands.entity(event.piece).remove::<Royal>();

            // TODO: Why is this hack necessary?
            // Without this, the position update is not replicated to clients.
            commands.entity(event.piece).remove::<Position>();
            commands
                .entity(event.piece)
                .insert(Position(query.get(event.piece).unwrap().0));

            commands.entity(event.piece).insert(mutated_piece.identity);

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

            if let Some(mutation_behavior) = &mutated_piece.behaviors.relay {
                commands
                    .entity(event.piece)
                    .insert(mutation_behavior.clone());
            }
        }
    }
}

pub(super) fn set_last_move(
    mut commands: Commands,
    mut turn_reader: EventReader<TurnEvent>,
    mut boards: Query<(Entity, Option<&mut LastAction>), With<Board>>,
    mut games: Query<(Entity, Option<&mut LastAction>), With<Game>>,
) {
    for event in turn_reader.read() {
        for (entity, maybe_move) in boards
            .get_mut(event.board)
            .into_iter()
            .chain(games.get_mut(event.game).into_iter())
        {
            if let Some(mut last_move) = maybe_move {
                last_move.0 = event.action.clone();
            } else {
                commands
                    .entity(entity)
                    .insert(LastAction(event.action.clone()));
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn end_turn(
    mut players_query: Query<(Entity, &InGame, Option<&mut Clock>, Option<&HasTurn>), With<Client>>,
    mut commands: Commands,
    mut turn_reader: EventReader<TurnEvent>,
) {
    for event in turn_reader.read() {
        for (player, in_game, clock, my_turn) in players_query.iter_mut() {
            if event.game != in_game.0 {
                continue;
            }
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
    } in turn_reader.read()
    {
        let Ok((mut game_ply, mut history)) = game_query.get_mut(*game) else {
            continue;
        };
        // TODO: in a future with 4-player, does this lead to bugs?
        if *game_ply != *ply {
            #[cfg(feature = "log")]
            bevy_log::warn!(
                "Turn ply {:?} does not match current game ply {:?}",
                *ply,
                *game_ply
            );
            continue;
        }
        history.push(*piece, action.clone());
        game_ply.increment();
    }
}

pub(super) fn detect_gameover(
    mut commands: Commands,
    game_query: Query<(Entity, &WinCondition), IsActiveGame>,
    royal_query: Query<(&InGame, &Team, Option<&Position>), With<Royal>>,
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
                    #[cfg(feature = "log")]
                    bevy_log::info!("Game {game_entity} over! Winner: Black");
                    commands
                        .entity(game_entity)
                        .insert(GameOver::new(Team::Black));
                }
                if all_captured(Team::Black) {
                    #[cfg(feature = "log")]
                    bevy_log::info!("Game {game_entity} over! Winner: White");
                    commands
                        .entity(game_entity)
                        .insert(GameOver::new(Team::White));
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
                    #[cfg(feature = "log")]
                    bevy_log::info!("Game {game_entity} over! Winner: Black");
                    commands
                        .entity(game_entity)
                        .insert(GameOver::new(Team::Black));
                }
                if any_captured(Team::Black) {
                    #[cfg(feature = "log")]
                    bevy_log::info!("Game {game_entity} over! Winner: White");
                    commands
                        .entity(game_entity)
                        .insert(GameOver::new(Team::White));
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

pub(super) fn spawn_game_entities(
    mut commands: Commands,
    query: Query<(Entity, &PieceSet, &GameBoard), Added<Game>>,
) {
    for (game_entity, piece_set, game_board) in query.iter() {
        #[cfg(feature = "log")]
        bevy_log::info!("Spawning pieces for game {:?}", game_entity);

        // add move history to the game
        commands
            .entity(game_entity)
            .insert((Ply::default(), ActionHistory::default()));

        // create an entity to manage board properties
        let board = match game_board {
            GameBoard::Chess => Board::chess_board(),
        };
        // TODO: Some sort of board bundle?
        let board_entity = commands
            .spawn((
                board,
                InGame(game_entity),
                Name::new(format!("Board (Game {:?})", game_entity)),
                BoardPieceCache::default(),
                BoardThreatsCache::default(),
                Replicated,
            ))
            .id();

        // spawn all game pieces
        for team in [Team::White, Team::Black].into_iter() {
            for PieceSpecification {
                piece,
                start_square,
            } in piece_set.0.iter()
            {
                let start_square = start_square.reorient(team.orientation(), &board);
                let name = Name::new(format!("{:?} {}-{:?}", team, start_square, piece.identity));
                #[cfg(feature = "log")]
                bevy_log::info!("...spawning piece: {}", name);

                let mut piece_builder = commands.spawn((
                    name,
                    piece.identity,
                    PieceBundle::new(start_square.into(), team),
                    InGame(game_entity),
                    OnBoard(board_entity),
                    History::<Position>::default(),
                    Replicated,
                ));

                if piece.royal.is_some() {
                    piece_builder.insert(Royal);
                }
                if let Some(mutation) = &piece.mutation {
                    piece_builder.insert(mutation.clone());
                }
                if let Some(behavior) = &piece.behaviors.pattern {
                    piece_builder.insert(behavior.clone());
                }
                if let Some(behavior) = &piece.behaviors.relay {
                    piece_builder.insert(behavior.clone());
                }
                if let Some(behavior) = piece.behaviors.en_passant {
                    piece_builder.insert(behavior);
                }
                if let Some(behavior) = piece.behaviors.castling {
                    piece_builder.insert(behavior);
                }
                if let Some(behavior) = piece.behaviors.castling_target {
                    piece_builder.insert(behavior);
                }
            }
        }
    }
}
