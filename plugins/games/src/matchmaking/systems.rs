use itertools::Itertools;

use bevy_ecs::prelude::{
    Commands, Entity, EventReader, Query, RemovedComponents, ResMut, With, Without,
};
use bevy_replicon::prelude::{ClientId, ConnectedClients, FromClient};

use replication::Client;

use crate::{
    components::{
        GameRequest, GameRequestBundle, GameRequestClock, GameRequestVariant, InGame, Player,
        SpawnGame,
    },
    gameplay::components::Game,
};

use super::{GameOpponent, LeaveGameEvent, RequestJoinGameEvent};

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
                let variant = event.event.game.unwrap_or_default();
                let clock = event.event.clock.as_ref();
                let spawn_game = SpawnGame::new(variant.piece_set())
                    .with_clock(clock.map(|requested_clock| requested_clock.to_clock()));
                #[cfg(feature = "log")]
                bevy_log::info!(
                    "Starting a local game with variant {variant:?} and clock {clock:?}"
                );
                commands.trigger(spawn_game);
            }
            GameOpponent::AgainstBot | GameOpponent::Analysis => {
                unimplemented!("Can't play games against bots yet :(");
            }
        }
    }
}

pub(super) fn handle_leave_events(
    mut commands: Commands,
    mut leave_requests: EventReader<FromClient<LeaveGameEvent>>,
    players: Query<(Entity, &Client)>,
) {
    for event in leave_requests.read() {
        if let Some((entity, client)) = players
            .iter()
            .find(|(_, client)| client.id == event.client_id)
        {
            #[cfg(feature = "log")]
            bevy_log::info!(
                "Player {} has left the game. Removing player entity {entity}",
                client.id.get()
            );

            commands.entity(entity).remove::<InGame>();
        }
    }
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
#[allow(clippy::type_complexity)]
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
            bevy_log::info!(
                "Starting online game for players {:?} and {:?}",
                entity1,
                entity2
            );

            matched_entities.push(entity1);
            matched_entities.push(entity2);

            let pieces = variant
                .unwrap_or(GameRequestVariant::FeaturedGameOne)
                .piece_set();
            let spawn_game = SpawnGame::new(pieces)
                .with_players(entity1, entity2)
                .with_clock(clock.map(|requested_clock| requested_clock.to_clock()));
            commands.trigger(spawn_game);
        }
    }

    for entity in matched_entities {
        commands.entity(entity).remove::<GameRequestBundle>();
    }
}

pub(super) fn despawn_empty_games(
    mut commands: Commands,
    games: Query<Entity, With<Game>>,
    players: Query<&InGame, With<Player>>,
) {
    for game in games.iter() {
        if !players.iter().any(|in_game| in_game.0 == game) {
            #[cfg(feature = "log")]
            bevy_log::info!("Players not found for game {game}: Despawning game");
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
        #[cfg(feature = "log")]
        bevy_log::info!("Game {game} removed; despawning entities:");
        for (entity, _) in game_entities
            .iter()
            .filter(|(_, in_game)| in_game.0 == game)
        {
            #[cfg(feature = "log")]
            bevy_log::info!("...despawning {entity}");
            commands.entity(entity).despawn();
        }
    }
}

// TODO: there are probably some visibility bugs right now
pub(super) fn handle_visibility(
    players: Query<(Entity, Option<&Client>, Option<&InGame>), With<Player>>,
    game_entities: Query<(Entity, &InGame), Without<Player>>,
    mut connected_clients: ResMut<ConnectedClients>,
) {
    // let players have visibility over all entities present in the same game
    for (entity, player, player_game) in players
        .iter()
        .filter_map(|(entity, player, in_game)| in_game.map(|game| (entity, player, game)))
    {
        let client_id = player.map(|client| client.id).unwrap_or(ClientId::SERVER);
        let client = connected_clients.client_mut(client_id);
        let visibility = client.visibility_mut();

        let client_id = client_id.get();
        #[cfg(feature = "log")]
        bevy_log::info!("Handling visibility for client {client_id}:");

        // player can see themselves
        visibility.set_visibility(entity, true);
        #[cfg(feature = "log")]
        bevy_log::info!("...CAN see its player entity {entity}");

        // and the game instance
        // TODO: turning this off when switching / ending games?
        visibility.set_visibility(player_game.0, true);
        #[cfg(feature = "log")]
        bevy_log::info!("...CAN see its game entity {}", player_game.0);

        // player can see other game entities
        for (entity, in_game) in game_entities.iter() {
            if in_game == player_game {
                #[cfg(feature = "log")]
                bevy_log::info!("...CAN see {entity}");
                visibility.set_visibility(entity, true);
            } else {
                #[cfg(feature = "log")]
                bevy_log::info!("...CANNOT see {entity}");
                visibility.set_visibility(entity, false);
            }
        }
    }

    #[cfg(feature = "log")]
    if players.iter().count() > 1 {
        bevy_log::info!("Now handling visibility between clients:");
    }

    // players also need to be able to see each other when either both in lobby, or both in the same game
    for [(entity1, player1, in_game1), (entity2, player2, in_game2)] in players.iter_combinations()
    {
        let visible = match (in_game1, in_game2) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(game1), Some(game2)) => game1 == game2,
        };

        let client1_id = player1.map(|client| client.id).unwrap_or(ClientId::SERVER);
        let client1 = connected_clients.client_mut(client1_id);
        let visibility1 = client1.visibility_mut();
        visibility1.set_visibility(entity2, visible);

        let client2_id = player2.map(|client| client.id).unwrap_or(ClientId::SERVER);
        let client2 = connected_clients.client_mut(client2_id);
        let visibility2 = client2.visibility_mut();
        visibility2.set_visibility(entity1, visible);

        #[cfg(feature = "log")]
        {
            let client1_id = client1_id.get();
            let client2_id = client2_id.get();
            bevy_log::info!(
                "Clients {client1_id} (entity {entity1}) and {client2_id} (entity {entity2}) see each other",
            );
        }
    }
}
