use bevy::prelude::{Commands, Entity, EventReader, EventWriter, Query, With};

use bevy_replicon::prelude::{ClientId, FromClient, SendMode, ToClients};

use chess::{
    board::{Board, OnBoard},
    pieces::{Mutation, MutationCondition, Position, Royal},
    team::Team,
};

use crate::{
    components::{Client, CurrentTurn, InGame, IsActiveGame, Ply, WinCondition},
    gameplay::components::GameOver,
};

use super::{PlayTurn, RequestTurnEvent, RequireMutationEvent};

pub(super) fn trigger_turns(
    mut commands: Commands,
    game_query: Query<(&Ply, &CurrentTurn), IsActiveGame>,
    board_query: Query<&Board>,
    player_query: Query<(&Team, &InGame, Option<&Client>)>,
    piece_query: Query<(&Team, &OnBoard, Option<&Mutation>)>,
    mut requested_turns: EventReader<FromClient<RequestTurnEvent>>,
    mut require_mutation_writer: EventWriter<ToClients<RequireMutationEvent>>,
) {
    for FromClient {
        event:
            RequestTurnEvent {
                piece,
                game,
                action,
                promotion,
            },
        client_id,
    } in requested_turns.read()
    {
        // is there a game instance?
        let Ok((ply, current_turn)) = game_query.get(*game) else {
            bevy::log::warn!("Failed to find game data for {game}");
            continue;
        };
        // does the selected piece exist?
        let Ok((piece_team, on_board, mutation)) = piece_query.get(*piece) else {
            bevy::log::warn!("Failed to find piece data for {piece}");
            continue;
        };
        // get the player data
        let Some((player_team, in_game, player)) =
            player_query.iter().find(|(player_team, _, player)| {
                player.map(|client| client.id).unwrap_or(ClientId::SERVER) == *client_id
                    && **player_team == current_turn.0
            })
        else {
            bevy::log::warn!(
                "Failed to find player for ClientId {client_id:?} that can play for team {:?}",
                current_turn.0
            );
            continue;
        };
        // is the piece controlled by the player?
        if piece_team != player_team {
            bevy::log::warn!(
                "Piece {piece} is controlled by team {piece_team:?}, not team {player_team:?}",
            );
            continue;
        }

        let mut turn = None;
        if let Some(mutation) = mutation {
            let Ok(board) = board_query.get(on_board.0) else {
                bevy::log::warn!("Failed to find board {}", on_board.0);
                continue;
            };
            match mutation.condition {
                MutationCondition::LocalRank(rank) => {
                    let reoriented_rank = action
                        .movement
                        .to
                        .reorient(piece_team.orientation(), board)
                        .rank;
                    if rank != reoriented_rank {
                        turn = Some(PlayTurn::action(
                            *ply,
                            *piece,
                            on_board.0,
                            in_game.0,
                            action.clone(),
                        ));
                    } else if mutation.to_piece.len() == 1 {
                        turn = Some(PlayTurn::mutation(
                            *ply,
                            *piece,
                            on_board.0,
                            in_game.0,
                            action.clone(),
                            mutation.to_piece.first().unwrap().clone(),
                        ));
                    } else if let Some(promotion) = promotion {
                        turn = Some(PlayTurn::mutation(
                            *ply,
                            *piece,
                            on_board.0,
                            in_game.0,
                            action.clone(),
                            promotion.clone(),
                        ));
                    } else {
                        require_mutation_writer.send(ToClients {
                            mode: player
                                .map(|player| SendMode::Direct(player.id))
                                .unwrap_or(SendMode::Broadcast),
                            event: RequireMutationEvent {
                                piece: *piece,
                                game: *game,
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
            turn = Some(PlayTurn::action(
                *ply,
                *piece,
                on_board.0,
                in_game.0,
                action.clone(),
            ));
        }

        if let Some(turn) = turn {
            commands.trigger(turn);
        }
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
                    bevy::log::info!("Game {game_entity} over! Winner: Black");
                    commands
                        .entity(game_entity)
                        .insert(GameOver::new(Team::Black));
                }
                if all_captured(Team::Black) {
                    bevy::log::info!("Game {game_entity} over! Winner: White");
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
                    bevy::log::info!("Game {game_entity} over! Winner: Black");
                    commands
                        .entity(game_entity)
                        .insert(GameOver::new(Team::Black));
                }
                if any_captured(Team::Black) {
                    bevy::log::info!("Game {game_entity} over! Winner: White");
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
