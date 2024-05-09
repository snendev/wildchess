use std::collections::HashMap;

use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::prelude::{Commands, Entity, EventReader, Query, Res, ResMut, Resource, With};

use bevy_renet2::{
    renet2::{ClientId, RenetServer, ServerEvent},
    RenetServerPlugin,
};

use crate::{
    connection_config, ClientChannel, NetworkedEntities, Player, PlayerCommand, ServerChannel,
    ServerMessages, PROTOCOL_ID,
};

// use renet2_visualizer::RenetServerVisualizer;

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<ClientId, Entity>,
}

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetServerPlugin);
        app.insert_resource(ServerLobby::default());
        // app.insert_resource(RenetServerVisualizer::<200>::default());

        #[cfg(any(
            feature = "web_transport_server",
            feature = "memory_transport",
            feature = "native_transport"
        ))]
        app.add_plugins(NativeServerTransportPlugin);

        #[cfg(feature = "steam_transport")]
        app.add_plugins(SteamServerTransportPlugin);

        app.add_systems(
            Update,
            (
                Self::server_handle_connections,
                Self::server_handle_inputs,
                Self::server_network_sync,
                // update_visualizer_system,
            ),
        );
    }
}

impl ServerPlugin {
    fn server_handle_connections(
        mut commands: Commands,
        mut server: ResMut<RenetServer>,
        mut lobby: ResMut<ServerLobby>,
        mut server_events: EventReader<ServerEvent>,
        players: Query<(Entity, &Player)>,
    ) {
        for event in server_events.read() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("Player {} connected.", client_id);
                    // visualizer.add_client(*client_id);

                    // Initialize other players for this new client
                    for (entity, player) in players.iter() {
                        let message = bincode::serialize(&ServerMessages::PlayerCreate {
                            id: player.id,
                            entity,
                        })
                        .unwrap();
                        server.send_message(*client_id, ServerChannel::ServerMessages, message);
                    }

                    // Spawn new player
                    let player_entity = commands.spawn(Player { id: *client_id }).id();

                    lobby.players.insert(*client_id, player_entity);

                    let message = bincode::serialize(&ServerMessages::PlayerCreate {
                        id: *client_id,
                        entity: player_entity,
                    })
                    .unwrap();
                    server.broadcast_message(ServerChannel::ServerMessages, message);
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("Player {} disconnected: {}", client_id, reason);
                    // visualizer.remove_client(*client_id);
                    if let Some(player_entity) = lobby.players.remove(client_id) {
                        commands.entity(player_entity).despawn();
                    }

                    let message =
                        bincode::serialize(&ServerMessages::PlayerRemove { id: *client_id })
                            .unwrap();
                    server.broadcast_message(ServerChannel::ServerMessages, message);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn server_handle_inputs(lobby: Res<ServerLobby>, mut server: ResMut<RenetServer>) {
        for client_id in server.clients_id() {
            while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
                let command: PlayerCommand = bincode::deserialize(&message).unwrap();
                match command {
                    PlayerCommand::FakeCommand { value } => {
                        println!("Received fake command from client {}: {}", client_id, value);

                        if let Some(player_entity) = lobby.players.get(&client_id) {
                            let message = ServerMessages::AckCommand {
                                value,
                                player: *player_entity,
                            };
                            let message = bincode::serialize(&message).unwrap();
                            server.broadcast_message(ServerChannel::ServerMessages, message);
                        }
                    }
                }
            }

            // N.B. alternative for messages by insert:

            // while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            //     let input: PlayerInput = bincode::deserialize(&message).unwrap();
            //     if let Some(player_entity) = lobby.players.get(&client_id) {
            //         commands.entity(*player_entity).insert(input);
            //     }
            // }
        }
    }

    // fn update_visualizer_system(
    //     mut egui_contexts: EguiContexts,
    //     mut visualizer: ResMut<RenetServerVisualizer<200>>,
    //     server: Res<RenetServer>,
    // ) {
    //     visualizer.update(&server);
    //     visualizer.show_window(egui_contexts.ctx_mut());
    // }

    #[allow(clippy::type_complexity)]
    fn server_network_sync(mut server: ResMut<RenetServer>, query: Query<Entity, With<Player>>) {
        let mut networked_entities = NetworkedEntities::default();
        for entity in query.iter() {
            networked_entities.entities.push(entity);
        }

        let sync_message = bincode::serialize(&networked_entities).unwrap();
        server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
    }
}

#[cfg(any(
    feature = "web_transport_server",
    feature = "memory_transport",
    feature = "native_transport"
))]
struct NativeServerTransportPlugin;

#[cfg(any(
    feature = "web_transport_server",
    feature = "memory_transport",
    feature = "native_transport"
))]
impl Plugin for NativeServerTransportPlugin {
    fn build(&self, app: &mut App) {
        use bevy_renet2::renet2::transport::{
            NetcodeServerTransport, ServerAuthentication, ServerSetupConfig,
        };
        use bevy_renet2::transport::NetcodeServerPlugin;
        use std::time::SystemTime;

        app.add_plugins(NetcodeServerPlugin);

        let server = RenetServer::new(connection_config());
        app.insert_resource(server);

        let public_addr = "127.0.0.1:5000".parse().unwrap();

        let current_time: std::time::Duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let server_config = ServerSetupConfig {
            current_time,
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            socket_addresses: vec![vec![public_addr]],
            authentication: ServerAuthentication::Unsecure,
        };

        #[cfg(feature = "native_transport")]
        let socket = {
            let udp_socket = std::net::UdpSocket::bind(public_addr).unwrap();
            renet2::transport::NativeSocket::new(udp_socket).unwrap()
        };

        #[cfg(feature = "web_transport_server")]
        let socket = {
            #[derive(Resource)]
            pub struct TokioRuntime(#[allow(dead_code)] tokio::runtime::Runtime);

            use base64::Engine;
            let (config, cert_hash) =
                renet2::transport::WebTransportServerConfig::new_selfsigned(public_addr, 4);

            let cert_hash_b64 =
                base64::engine::general_purpose::STANDARD.encode(cert_hash.hash.as_ref());
            println!(
                "WT SERVER CERT HASH (PASTE ME TO CLIENTS): {:?}",
                cert_hash_b64
            );
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let socket =
                renet2::transport::WebTransportServer::new(config, runtime.handle().clone())
                    .unwrap();
            app.insert_resource(TokioRuntime(runtime));
            socket
        };

        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
        app.insert_resource(transport);
    }
}

// TODO: untested
#[cfg(feature = "steam_transport")]
struct SteamServerTransportPlugin;

#[cfg(feature = "steam_transport")]
impl Plugin for SteamServerTransportPlugin {
    fn build(&self, app: &mut App) {
        use bevy_app::PreUpdate;
        use renet2_steam::bevy::{SteamServerConfig, SteamServerPlugin, SteamServerTransport};
        use renet2_steam::AccessPermission;

        let (steam_client, single) = steamworks::Client::init_app(480).unwrap();

        let server: RenetServer = RenetServer::new(connection_config());

        let steam_transport_config = SteamServerConfig {
            max_clients: 10,
            access_permission: AccessPermission::Public,
        };
        let transport = SteamServerTransport::new(&steam_client, steam_transport_config).unwrap();

        app.add_plugins(SteamServerPlugin);
        app.insert_resource(server);
        app.insert_non_send_resource(transport);
        app.insert_non_send_resource(single);

        app.add_systems(PreUpdate, Self::steam_callbacks);
    }
}

#[cfg(feature = "steam_transport")]
impl SteamServerTransportPlugin {
    fn steam_callbacks(client: bevy_ecs::prelude::NonSend<steamworks::SingleClient>) {
        client.run_callbacks();
    }
}
