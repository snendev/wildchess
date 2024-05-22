use bevy_core::Name;
use bevy_ecs::prelude::{
    Added, Changed, Commands, Entity, EventReader, Query, RemovedComponents, ResMut, With, Without,
};

use bevy_replicon::{
    core::replication_rules::Replication, prelude::FromClient,
    server::connected_clients::ConnectedClients,
};

use chess::{
    behavior::{BoardPieceCache, BoardThreatsCache},
    board::{Board, OnBoard},
    pieces::{PieceBundle, Position, Royal},
    team::Team,
};
use layouts::{FeaturedWildLayout, PieceSpecification, RandomWildLayout};
use replication::Player;

use crate::{
    components::{ActionHistory, ClockConfiguration, GameBoard, HasTurn, History, InGame, Ply},
    gameplay::components::{Game, GameSpawner, PieceSet, WinCondition},
};

use super::{
    components::{GameRequest, GameRequestBundle, GameRequestClock, GameRequestVariant},
    GameOpponent, RequestJoinGameEvent,
};

pub(super) fn handle_game_requests(
    mut commands: Commands,
    mut join_requests: EventReader<FromClient<RequestJoinGameEvent>>,
    players: Query<(Entity, &Player)>,
) {
    let mut requests_iter = join_requests.read();
    while let Some(event) = requests_iter.next() {
        match event.event.opponent {
            GameOpponent::Online => {
                let Some((player, _)) = players
                    .iter()
                    .find(|(_, player)| player.id == event.client_id)
                else {
                    #[cfg(feature = "log")]
                    bevy_log::info!(
                        "Join request received from player without player entity! Client {}",
                        event.client_id.get()
                    );
                    continue;
                };
                #[cfg(feature = "log")]
                bevy_log::info!(
                    "Player {:?} (client {}) seeking match...",
                    player,
                    event.client_id.get()
                );
                let mut player_builder = commands.entity(player);
                player_builder.insert(GameRequest);
                if let Some(game) = event.event.game {
                    player_builder.insert(game);
                }
                if let Some(clock) = event.event.clock {
                    player_builder.insert(clock);
                }
            }
            GameOpponent::Local | GameOpponent::AgainstBot | GameOpponent::Analysis => {
                unimplemented!("Can't play games against bots yet :(");
            }
        }
    }
}

// match the most specified game requests first since they will be less easy to match
pub(super) fn match_specified_game_requests(
    mut commands: Commands,
    specified_game_request: Query<
        (Entity, &GameRequestVariant, &GameRequestClock),
        With<GameRequest>,
    >,
) {
    let mut matched_entities: Vec<Entity> = vec![];

    // first match players with the most specific requests
    for [(entity1, variant1, clock1), (entity2, variant2, clock2)] in
        specified_game_request.iter_combinations()
    {
        if matched_entities.contains(&entity1) || matched_entities.contains(&entity2) {
            continue;
        }
        if variant1 == variant2 && clock1 == clock2 {
            #[cfg(feature = "log")]
            bevy_log::info!("Spawning game for players {:?} and {:?}", entity1, entity2);

            matched_entities.push(entity1);
            matched_entities.push(entity2);

            let piece_set = PieceSet(match variant1 {
                GameRequestVariant::FeaturedGameOne => FeaturedWildLayout::One.pieces(),
                GameRequestVariant::FeaturedGameTwo => FeaturedWildLayout::Two.pieces(),
                GameRequestVariant::FeaturedGameThree => FeaturedWildLayout::Three.pieces(),
                GameRequestVariant::Wild => RandomWildLayout::pieces(),
            });
            // TODO: incorporate featured boards
            let game =
                GameSpawner::new_game(GameBoard::Chess, piece_set, WinCondition::RoyalCapture)
                    .with_clock(clock1.to_clock())
                    .spawn(&mut commands);

            commands.entity(entity1).insert(InGame(game));
            commands.entity(entity2).insert(InGame(game));
        }
    }

    for entity in matched_entities {
        commands.entity(entity).remove::<GameRequestBundle>();
    }
}

// runs after and identical to `match_specified_game_requests`, but checking requests that do not
// specify both variant and clock settings.
pub(super) fn match_remaining_game_requests(
    mut commands: Commands,
    game_requests: Query<
        (
            Entity,
            Option<&GameRequestVariant>,
            Option<&GameRequestClock>,
        ),
        With<GameRequest>,
    >,
) {
    let mut matched_entities: Vec<Entity> = vec![];

    // first match players with the most specific requests
    for [(entity1, variant1, clock1), (entity2, variant2, clock2)] in
        game_requests.iter_combinations()
    {
        if matched_entities.contains(&entity1) || matched_entities.contains(&entity2) {
            continue;
        }

        // properties do not match iff they are both specified and they differ
        // otherwise they are a match
        let variants_are_matching = variant1
            .zip(variant2)
            .map(|(v1, v2)| *v1 == *v2)
            .unwrap_or(true);
        if !variants_are_matching {
            continue;
        }
        let clocks_are_matching = clock1
            .zip(clock2)
            .map(|(c1, c2)| *c1 == *c2)
            .unwrap_or(true);
        if !clocks_are_matching {
            continue;
        }

        #[cfg(feature = "log")]
        bevy_log::info!("Spawning game for players {:?} and {:?}", entity1, entity2);

        let variant = variant1
            .or(variant2)
            .unwrap_or(&GameRequestVariant::FeaturedGameOne);
        let clock = clock1.or(clock2).unwrap_or(&GameRequestClock::Rapid);

        matched_entities.push(entity1);
        matched_entities.push(entity2);

        let piece_set = PieceSet(match variant {
            GameRequestVariant::FeaturedGameOne => FeaturedWildLayout::One.pieces(),
            GameRequestVariant::FeaturedGameTwo => FeaturedWildLayout::Two.pieces(),
            GameRequestVariant::FeaturedGameThree => FeaturedWildLayout::Three.pieces(),
            GameRequestVariant::Wild => RandomWildLayout::pieces(),
        });

        // TODO: incorporate featured boards
        let game = GameSpawner::new_game(GameBoard::Chess, piece_set, WinCondition::RoyalCapture)
            .with_clock(clock.to_clock())
            .spawn(&mut commands);

        commands.entity(entity1).insert(InGame(game));
        commands.entity(entity2).insert(InGame(game));
    }

    for entity in matched_entities {
        commands.entity(entity).remove::<GameRequestBundle>();
    }
}

// TODO: Give Players an `OnBoard` as well
pub(super) fn assign_game_teams(
    mut commands: Commands,
    players: Query<(Entity, &InGame), (With<Player>, Without<Team>, Added<InGame>)>,
    games: Query<Option<&ClockConfiguration>, With<Game>>,
) {
    'outer: for chunk in players
        .iter()
        .collect::<Vec<_>>()
        .chunk_by(|(_, game1), (_, game2)| **game1 == **game2)
    {
        for ((entity, in_game), team) in chunk.iter().zip([Team::White, Team::Black]) {
            let Ok(clock) = games.get(in_game.0) else {
                continue 'outer;
            };
            #[cfg(feature = "log")]
            bevy_log::info!(
                "Setting player {:?} to team {:?} in game {:?}",
                entity,
                team,
                in_game.0
            );
            let mut builder = commands.entity(*entity);
            builder.insert((team, team.orientation()));
            if team == Team::White {
                builder.insert(HasTurn);
            }
            if let Some(clock) = clock {
                builder.insert(clock.clone());
            }
        }
    }
}

// TODO: this system belongs in the other plugin
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
                Replication,
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

                let mut piece_builder = commands.spawn((
                    name,
                    piece.identity,
                    PieceBundle::new(start_square.into(), team),
                    InGame(game_entity),
                    OnBoard(board_entity),
                    History::<Position>::default(),
                    Replication,
                ));

                if piece.royal.is_some() {
                    piece_builder.insert(Royal);
                }
                if let Some(mutation) = &piece.mutation {
                    piece_builder.insert(mutation.clone());
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

pub(super) fn despawn_empty_games(
    mut commands: Commands,
    games: Query<Entity, With<Game>>,
    players: Query<&InGame, With<Player>>,
) {
    for game in games.iter() {
        if players.iter().find(|in_game| in_game.0 == game).is_none() {
            commands.entity(game).despawn();
        }
    }
}

pub(super) fn cleanup_game_entities(
    mut commands: Commands,
    mut removed_games: RemovedComponents<Game>,
    game_entities: Query<(Entity, &InGame)>,
) {
    for game in removed_games.read() {
        for (entity, _) in game_entities
            .iter()
            .filter(|(_, in_game)| in_game.0 == game)
        {
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn handle_visibility(
    players_not_in_game: Query<(Entity, &Player), Without<InGame>>,
    players: Query<(Entity, &Player, &InGame), Changed<InGame>>,
    game_entities: Query<(Entity, &InGame), Without<Player>>,
    mut connected_clients: ResMut<ConnectedClients>,
) {
    // players not in game can see all other players in lobby
    for [(entity1, player1), (entity2, player2)] in players_not_in_game.iter_combinations() {
        let client1 = connected_clients.client_mut(player1.id);
        let visibility1 = client1.visibility_mut();
        visibility1.set_visibility(entity2, true);

        let client2 = connected_clients.client_mut(player2.id);
        let visibility2 = client2.visibility_mut();
        visibility2.set_visibility(entity1, true);
    }

    // let players have visibility over all entities present in the same game
    for (entity, player, player_game) in players.iter() {
        let client = connected_clients.client_mut(player.id);
        let visibility = client.visibility_mut();

        // player can see themselves
        visibility.set_visibility(entity, true);
        // and the game instance
        // TODO: turning this off when switching / ending games?
        visibility.set_visibility(player_game.0, true);

        // player can see other game entities
        for (entity, in_game) in game_entities.iter() {
            if in_game == player_game {
                visibility.set_visibility(entity, true);
            } else {
                visibility.set_visibility(entity, false);
            }
        }
    }
    // let players have visibility of other players present in the same game
    for [(entity1, player1, game1), (entity2, player2, game2)] in players.iter_combinations() {
        let is_visible = game1 == game2;

        let client1 = connected_clients.client_mut(player1.id);
        let visibility1 = client1.visibility_mut();
        visibility1.set_visibility(entity2, is_visible);

        let client2 = connected_clients.client_mut(player2.id);
        let visibility2 = client2.visibility_mut();
        visibility2.set_visibility(entity1, is_visible);
    }
}
