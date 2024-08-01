use bevy_app::prelude::{Plugin, PluginGroup, Startup, Update};
use bevy_core::Name;
use bevy_ecs::prelude::{Commands, Entity, EventReader, Query, Res};

use bevy_replicon::prelude::{
    Replicated, RepliconChannels, RepliconCorePlugin, RepliconPlugins, ServerEvent, ServerPlugin,
    VisibilityPolicy,
};
use bevy_replicon_renet2::{
    renet2::{ConnectionConfig, RenetServer},
    RenetChannelsExt,
};

use crate::Client;

pub struct ServerReplicationPlugin;

impl Plugin for ServerReplicationPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        if !app.is_plugin_added::<RepliconCorePlugin>() {
            app.add_plugins(RepliconPlugins.build().disable::<ServerPlugin>());
        }
        app.add_plugins(ServerPlugin {
            visibility_policy: VisibilityPolicy::Whitelist,
            ..Default::default()
        })
        .add_systems(Startup, Self::start_server)
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
            ..Default::default()
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
                    #[cfg(feature = "log")]
                    bevy_log::info!("Player {} connected.", client_id.get());
                    // Spawn new player entity
                    commands.spawn((
                        Replicated,
                        Name::new(format!("Player {}", client_id.get())),
                        Client { id: *client_id },
                    ));
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    if let Some((player_entity, _)) =
                        clients.iter().find(|(_, Client { id })| *id == *client_id)
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
