use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::prelude::{Commands, Entity, EventReader, Query};

use bevy_renet2::{renet2::RenetServer, RenetServerPlugin};
use bevy_replicon::{core::replication_rules::Replication, prelude::ServerEvent};

use crate::{connection_config, Player, PROTOCOL_ID};

// use renet2_visualizer::RenetServerVisualizer;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetServerPlugin);

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
                Self::handle_connections,
                // Self::server_handle_inputs,
            ),
        );
    }
}

impl ServerPlugin {
    fn handle_connections(
        mut commands: Commands,
        mut server_events: EventReader<ServerEvent>,
        players: Query<(Entity, &Player)>,
    ) {
        for event in server_events.read() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    #[cfg(feature = "log")]
                    bevy_log::info!("Player {} connected.", client_id.get());
                    // Spawn new player entity
                    commands.spawn((Replication, Player { id: *client_id }));
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    if let Some((player_entity, _)) =
                        players.iter().find(|(_, Player { id })| *id == *client_id)
                    {
                        #[cfg(feature = "log")]
                        bevy_log::debug!("Player disconnected: {}", reason);
                        commands.entity(player_entity).despawn();
                    }
                }
            }
        }
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
