use bevy::prelude::{
    App, Commands, Entity, EventReader, Name, Plugin, Query, Res, Startup, Update,
};

use bevy_replicon::prelude::{Replicated, RepliconChannels, ServerEvent};
use bevy_replicon_renet2::{
    renet2::{ConnectionConfig, RenetServer},
    RenetChannelsExt,
};

use games::components::Client;

pub struct ServerReplicationPlugin;

impl Plugin for ServerReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::start_server)
            .add_systems(Update, Self::handle_connections);
    }
}

impl ServerReplicationPlugin {
    fn start_server(mut commands: Commands, replicon_channels: Res<RepliconChannels>) {
        let server_channels_config = replicon_channels.get_server_configs();
        let client_channels_config = replicon_channels.get_client_configs();

        let server = RenetServer::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            available_bytes_per_tick: 60_000,
        });
        commands.insert_resource(server);
    }

    fn handle_connections(
        mut commands: Commands,
        mut server_events: EventReader<ServerEvent>,
        clients: Query<(Entity, &Client)>,
    ) {
        for event in server_events.read() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    bevy::log::info!("Player {} connected.", client_id.get());
                    // Spawn new player entity
                    commands.spawn((
                        Replicated,
                        Name::new(format!("Player {}", client_id.get())),
                        Client { id: *client_id },
                    ));
                }
                ServerEvent::ClientDisconnected {
                    client_id,
                    reason: _reason,
                } => {
                    if let Some((player_entity, _)) =
                        clients.iter().find(|(_, Client { id })| *id == *client_id)
                    {
                        bevy::log::debug!("Player disconnected: {}", _reason);
                        commands.entity(player_entity).despawn();
                    }
                }
            }
        }
    }
}
