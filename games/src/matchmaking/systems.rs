use bevy_core::Name;
use bevy_ecs::prelude::{
    Added, Commands, Entity, EventReader, Query, RemovedComponents, ResMut, With, Without,
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
use itertools::Itertools;
use layouts::{FeaturedWildLayout, PieceSpecification, RandomWildLayout};
use replication::Client;

use crate::{
    components::{ActionHistory, ClockConfiguration, GameBoard, HasTurn, History, InGame, Ply},
    gameplay::components::{Game, GameSpawner, PieceSet, Player, WinCondition},
};

use super::{
    components::{GameRequest, GameRequestBundle, GameRequestClock, GameRequestVariant},
    GameOpponent, RequestJoinGameEvent,
};

pub(super) fn handle_game_requests(
    mut commands: Commands,
    mut join_requests: EventReader<FromClient<RequestJoinGameEvent>>,
    players: Query<(Entity, &Client)>,
) {
    for event in join_requests.read() {
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
            GameOpponent::Local => {
                let player1 = commands.spawn(Player).id();
                let player2 = commands.spawn(Player).id();
                spawn_game(
                    &mut commands,
                    player1,
                    player2,
                    &event.event.game.unwrap_or_default(),
                    event.event.clock.as_ref(),
                );
            }
            GameOpponent::AgainstBot | GameOpponent::Analysis => {
                unimplemented!("Can't play games against bots yet :(");
            }
        }
    }
}

pub(super) fn handle_leave_events(
    mut commands: Commands,
    mut leave_requests: EventReader<FromClient<RequestJoinGameEvent>>,
    players: Query<(Entity, &Client)>,
) {
    for event in leave_requests.read() {
        if let Some((entity, _)) = players
            .iter()
            .find(|(_, player)| player.id == event.client_id)
        {
            commands.entity(entity).remove::<InGame>();
        }
    }
}

fn spawn_game(
    commands: &mut Commands,
    player1: Entity,
    player2: Entity,
    variant: &GameRequestVariant,
    clock: Option<&GameRequestClock>,
) {
    let piece_set = PieceSet(match variant {
        GameRequestVariant::FeaturedGameOne => FeaturedWildLayout::One.pieces(),
        GameRequestVariant::FeaturedGameTwo => FeaturedWildLayout::Two.pieces(),
        GameRequestVariant::FeaturedGameThree => FeaturedWildLayout::Three.pieces(),
        GameRequestVariant::Wild => RandomWildLayout::pieces(),
    });

    let game = GameSpawner::new_game(GameBoard::Chess, piece_set, WinCondition::RoyalCapture);
    let game = if let Some(clock) = clock {
        game.with_clock(clock.to_clock())
    } else {
        game
    }
    .spawn(commands);

    commands.entity(player1).insert(InGame(game));
    commands.entity(player2).insert(InGame(game));
}

/// Compares combinations of tuples so that combinations with more "Some"s are handled first
fn cmp_combinations<T, U, V>(
    pair1: &[(T, Option<U>, Option<V>)],
    pair2: &[(T, Option<U>, Option<V>)],
) -> std::cmp::Ordering {
    /// a count for the number of `Some`s in this Option (either 1 or 0)
    fn count_some<O>(option: &Option<O>) -> usize {
        option.as_ref().map(|_| 1).unwrap_or(0)
    }

    // for each combination, count how many of options are `Some` per type
    let pair1_u_somes = count_some(&pair1[0].1) + count_some(&pair1[1].1);
    let pair1_v_somes = count_some(&pair1[0].2) + count_some(&pair1[1].2);
    let pair2_u_somes = count_some(&pair2[0].1) + count_some(&pair2[1].1);
    let pair2_v_somes = count_some(&pair2[0].2) + count_some(&pair2[1].2);

    // cmp how many `Some`s for U, then cmp how many `Some`s for V
    pair1_u_somes
        .cmp(&pair2_u_somes)
        .then(pair1_v_somes.cmp(&pair2_v_somes))
}

fn combine_equal<O: Copy + PartialEq>(
    option1: Option<&O>,
    option2: Option<&O>,
) -> Option<Option<O>> {
    match (option1, option2) {
        (Some(value1), Some(value2)) => {
            if value1 == value2 {
                Some(Some(*value1))
            } else {
                None
            }
        }
        (Some(value), None) | (None, Some(value)) => Some(Some(*value)),
        (None, None) => Some(None),
    }
}

// match the most specified game requests first since they will be less easy to match
pub(super) fn match_game_requests(
    mut commands: Commands,
    specified_game_request: Query<
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
    for [(entity1, variant1, clock1), (entity2, variant2, clock2)] in specified_game_request
        .iter_combinations()
        .sorted_by(|pair1, pair2| cmp_combinations(pair1, pair2))
    {
        if matched_entities.contains(&entity1) || matched_entities.contains(&entity2) {
            continue;
        }

        // because of the way the combinations are sorted, we know that this is still greedy even though we check xor first
        let variant = combine_equal(variant1, variant2);
        let clock = combine_equal(clock1, clock2);
        if let (Some(variant), Some(clock)) = (variant, clock) {
            #[cfg(feature = "log")]
            bevy_log::info!("Spawning game for players {:?} and {:?}", entity1, entity2);

            matched_entities.push(entity1);
            matched_entities.push(entity2);

            spawn_game(
                &mut commands,
                entity1,
                entity2,
                &variant.unwrap_or(GameRequestVariant::FeaturedGameOne),
                clock.as_ref(),
            );
        }
    }

    for entity in matched_entities {
        commands.entity(entity).remove::<GameRequestBundle>();
    }
}

// TODO: Give Players an `OnBoard` as well
pub(super) fn assign_game_teams(
    mut commands: Commands,
    players: Query<(Entity, &InGame), (With<Client>, Without<Team>, Added<InGame>)>,
    games: Query<Option<&ClockConfiguration>, With<Game>>,
) {
    for chunk in players
        .iter()
        .collect::<Vec<_>>()
        .chunk_by(|(_, game1), (_, game2)| **game1 == **game2)
    {
        for ((entity, in_game), team) in chunk.iter().zip([Team::White, Team::Black]) {
            let Ok(clock) = games.get(in_game.0) else {
                continue;
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
                builder.insert(clock.clock.clone());
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
    players: Query<&InGame, With<Client>>,
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

// TODO: there are probably some visibility bugs right now
pub(super) fn handle_visibility(
    players: Query<(Entity, &Client, Option<&InGame>)>,
    game_entities: Query<(Entity, &InGame), Without<Client>>,
    mut connected_clients: ResMut<ConnectedClients>,
) {
    // let players have visibility over all entities present in the same game
    for (entity, player, player_game) in players
        .iter()
        .filter_map(|(entity, player, in_game)| in_game.map(|game| (entity, player, game)))
    {
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

    // players also need to be able to see each other when either both in lobby, or both in the same game
    for [(entity1, player1, in_game1), (entity2, player2, in_game2)] in players.iter_combinations()
    {
        let visible = match (in_game1, in_game2) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(game1), Some(game2)) => {
                if game1 == game2 {
                    true
                } else {
                    false
                }
            }
        };
        let client1 = connected_clients.client_mut(player1.id);
        let visibility1 = client1.visibility_mut();
        visibility1.set_visibility(entity2, visible);

        let client2 = connected_clients.client_mut(player2.id);
        let visibility2 = client2.visibility_mut();
        visibility2.set_visibility(entity1, visible);
    }
}
