use std::collections::HashMap;

use bevy_app::prelude::{App, Plugin, Update};
use bevy_ecs::prelude::{
    Commands, Component, Entity, Event, EventReader, EventWriter, IntoSystemConfigs,
    IntoSystemSetConfigs, Res, ResMut, Resource, SystemSet,
};

use bevy_renet2::{
    client_connected,
    renet2::{ClientId, RenetClient},
    RenetClientPlugin,
};

#[cfg(feature = "visualizer")]
use renet2_visualizer::{RenetClientVisualizer, RenetVisualizerStyle};

use crate::{
    connection_config, ClientChannel, NetworkedEntities, Player, PlayerCommand, ServerChannel,
    ServerMessages, PROTOCOL_ID,
};

#[derive(Event)]
pub struct TestAckEvent(pub u16, pub Entity);

#[derive(Component)]
struct ControlledPlayer;

#[derive(Default, Resource)]
struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug)]
struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

#[derive(Debug, Default, Resource)]
struct ClientLobby {
    players: HashMap<ClientId, PlayerInfo>,
}

#[derive(Debug, Resource)]
struct CurrentClientId(u64);

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Connected;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RenetClientPlugin);
        app.add_event::<TestAckEvent>();

        app.insert_resource(ClientLobby::default());
        app.insert_resource(NetworkMapping::default());

        app.configure_sets(Update, Connected.run_if(client_connected));
        let client = RenetClient::new(connection_config());
        app.insert_resource(client);

        #[cfg(any(
            feature = "native_transport",
            feature = "memory_transport",
            feature = "web_transport_client"
        ))]
        app.add_plugins(NativeClientTransportPlugin);
        #[cfg(feature = "steam_transport")]
        app.add_plugins(SteamClientTransportPlugin);

        app.add_event::<PlayerCommand>().add_systems(
            Update,
            (
                // Self::client_send_input,
                Self::client_send_player_commands,
                Self::client_sync_players,
            )
                .in_set(Connected),
        );

        // #[cfg(feature = "visualizer")]
        // app.insert_resource(RenetClientVisualizer::<200>::new(
        //     RenetVisualizerStyle::default(),
        // ));

        // app.add_systems(
        //     Startup,
        //     (Self::setup_level, Self::setup_camera, Self::setup_target),
        // );
        // #[cfg(feature = "visualizer")]
        // app.add_systems(Update, Self::update_visulizer_system);
    }
}

impl ClientPlugin {
    // #[cfg(feature = "visualizer")]
    // fn update_visulizer_system(
    //     mut egui_contexts: EguiContexts,
    //     mut visualizer: ResMut<RenetClientVisualizer<200>>,
    //     client: Res<RenetClient>,
    //     mut show_visualizer: Local<bool>,
    //     keyboard_input: Res<ButtonInput<KeyCode>>,
    // ) {
    //     visualizer.add_network_info(client.network_info());
    //     if keyboard_input.just_pressed(KeyCode::F1) {
    //         *show_visualizer = !*show_visualizer;
    //     }
    //     if *show_visualizer {
    //         visualizer.show_window(egui_contexts.ctx_mut());
    //     }
    // }

    fn client_send_player_commands(
        mut player_commands: EventReader<PlayerCommand>,
        mut client: ResMut<RenetClient>,
    ) {
        for command in player_commands.read() {
            let command_message = bincode::serialize(command).unwrap();
            client.send_message(ClientChannel::Command, command_message);
        }
    }

    fn client_sync_players(
        mut commands: Commands,
        mut client: ResMut<RenetClient>,
        client_id: Res<CurrentClientId>,
        mut lobby: ResMut<ClientLobby>,
        mut network_mapping: ResMut<NetworkMapping>,
        mut events: EventWriter<TestAckEvent>,
    ) {
        let client_id = client_id.0;
        while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
            let server_message = bincode::deserialize(&message).unwrap();
            match server_message {
                ServerMessages::PlayerCreate {
                    id,
                    // translation,
                    entity,
                } => {
                    println!("Player {} connected.", id);
                    let mut client_entity = commands.spawn(Player { id });

                    if client_id == id.raw() {
                        client_entity.insert(ControlledPlayer);
                    }

                    let player_info = PlayerInfo {
                        server_entity: entity,
                        client_entity: client_entity.id(),
                    };
                    lobby.players.insert(id, player_info);
                    network_mapping.0.insert(entity, client_entity.id());
                }
                ServerMessages::PlayerRemove { id } => {
                    println!("Player {} disconnected.", id);
                    if let Some(PlayerInfo {
                        server_entity,
                        client_entity,
                    }) = lobby.players.remove(&id)
                    {
                        commands.entity(client_entity).despawn();
                        network_mapping.0.remove(&server_entity);
                    }
                }
                ServerMessages::AckCommand { value, player } => {
                    events.send(TestAckEvent(value, player));
                }
            }
        }

        while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
            let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

            for i in 0..networked_entities.entities.len() {
                if let Some(_entity) = network_mapping.0.get(&networked_entities.entities[i]) {
                    // let translation = networked_entities.translations[i].into();
                    // let transform = Transform {
                    //     translation,
                    //     ..Default::default()
                    // };
                    // commands.entity(*entity).insert(transform);
                }
            }
        }
    }
}

#[cfg(any(
    feature = "web_transport_client",
    feature = "memory_transport",
    feature = "native_transport"
))]
struct NativeClientTransportPlugin;

#[cfg(any(
    feature = "web_transport_client",
    feature = "memory_transport",
    feature = "native_transport"
))]
impl Plugin for NativeClientTransportPlugin {
    fn build(&self, app: &mut App) {
        use renet2::transport::{ClientAuthentication, NetcodeClientTransport};
        #[cfg(not(feature = "web_transport_client"))]
        use std::time::SystemTime;
        #[cfg(feature = "web_transport_client")]
        use wasm_timer::SystemTime;

        app.add_plugins(bevy_renet2::transport::NetcodeClientPlugin);

        let server_addr = "127.0.0.1:5000".parse().unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            socket_id: 0,
            server_addr,
            user_data: None,
        };

        #[cfg(feature = "native_transport")]
        let socket = {
            let udp_socket = std::net::UdpSocket::bind("127.0.0.1:0").unwrap();
            renet2::transport::NativeSocket::new(udp_socket).unwrap()
        };
        // TODO: to support this at this layer we would need to pass the client socket in from above
        // #[cfg(feature = "memory_transport")]
        // let socket = renet2::transport::MemorySocketClient::new(client_id as u16, client_memory_socket).unwrap();
        #[cfg(all(feature = "web_transport_client", target_family = "wasm"))]
        let socket = {
            use base64::Engine;
            use renet2::transport::{ServerCertHash, WebTransportClientConfig};

            const HASH_B64: &'static str = "06TmjhRpJYtgJ8hkKCXHbVkxODuiGkxItTdixP5hkf8=";
            let hash = base64::engine::general_purpose::STANDARD
                .decode(HASH_B64)
                .unwrap();
            let config = WebTransportClientConfig::new_with_certs(
                server_addr,
                Vec::from([ServerCertHash::try_from(hash).unwrap()]),
            );
            renet2::transport::WebTransportClient::new(config)
        };

        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

        app.insert_resource(transport);
        app.insert_resource(CurrentClientId(client_id));
    }
}

// TODO: untested
#[cfg(feature = "steam_transport")]
struct SteamClientTransportPlugin;

#[cfg(feature = "steam_transport")]
impl Plugin for SteamClientTransportPlugin {
    fn build(&self, app: &mut App) {
        use bevy_app::PreUpdate;
        use renet2_steam::bevy::{SteamClientPlugin, SteamClientTransport};
        use steamworks::SteamId;

        let (steam_client, single) = steamworks::Client::init_app(480).unwrap();
        steam_client.networking_utils().init_relay_network_access();

        app.add_plugins(SteamClientPlugin);
        app.insert_non_send_resource(single);
        app.add_systems(PreUpdate, Self::steam_callbacks);

        let args: Vec<String> = std::env::args().collect();
        let server_steam_id: u64 = args[1].parse().unwrap();
        let server_steam_id = SteamId::from_raw(server_steam_id);
        let transport = SteamClientTransport::new(&steam_client, &server_steam_id).unwrap();
        app.insert_resource(transport);
        app.insert_resource(CurrentClientId(steam_client.user().steam_id().raw()));
    }
}

#[cfg(feature = "steam_transport")]
impl SteamClientTransportPlugin {
    fn steam_callbacks(client: bevy_ecs::NonSend<steamworks::SingleClient>) {
        client.run_callbacks();
    }
}
