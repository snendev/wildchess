use serde::{Deserialize, Serialize};

use bevy_app::prelude::{Plugin, Startup, Update};
use bevy_core::Name;
use bevy_ecs::{
    prelude::{Commands, Component, Entity, EventReader, Query, Res, World},
    world::Command,
};

use bevy_replicon::prelude::{
    AppRuleExt, ClientId, ClientPlugin, ParentSyncPlugin, Replicated, RepliconChannels,
    RepliconCorePlugin, ServerEvent, ServerPlugin, VisibilityPolicy,
};
use bevy_replicon_renet2::{
    renet2::{ConnectionConfig, RenetClient, RenetServer},
    RenetChannelsExt, RepliconRenetClientPlugin, RepliconRenetServerPlugin,
};

pub use bevy_replicon as replicon;
pub use bevy_replicon_renet2 as replicon_renet2;

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
pub struct Client {
    pub id: ClientId,
}

pub enum ReplicationPlugin {
    Server,
    Client,
}

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        if !app.is_plugin_added::<RepliconCorePlugin>() {
            app.add_plugins((RepliconCorePlugin, ParentSyncPlugin));
        }

        app.replicate::<Client>();

        match self {
            ReplicationPlugin::Server => {
                app.add_plugins((
                    ServerPlugin {
                        visibility_policy: VisibilityPolicy::Whitelist,
                        ..Default::default()
                    },
                    RepliconRenetServerPlugin,
                ))
                .add_systems(Startup, start_server)
                .add_systems(Update, handle_connections);
            }
            ReplicationPlugin::Client => {
                app.add_plugins((ClientPlugin, RepliconRenetClientPlugin));
                ClientCommand::Connect.apply(app.world_mut());
            }
        }
    }
}

pub enum ClientCommand {
    Connect,
    Disconnect,
}

impl Command for ClientCommand {
    fn apply(self, world: &mut World) {
        match self {
            ClientCommand::Connect => {
                connect_to_server(world);
            }
            ClientCommand::Disconnect => {
                world.remove_resource::<RenetClient>();
            }
        }
    }
}

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

// TODO: turn this into a system once bevy_renet2 uses the run condition here
// https://github.com/UkoeHB/renet2/blob/main/bevy_renet2/src/lib.rs#L62
fn connect_to_server(world: &mut World) {
    let replicon_channels = world
        .get_resource::<RepliconChannels>()
        .expect("replicon plugins to be added before transport plugins");
    let server_channels_config = replicon_channels.get_server_configs();
    let client_channels_config = replicon_channels.get_client_configs();
    let client = RenetClient::new(ConnectionConfig {
        server_channels_config,
        client_channels_config,
        ..Default::default()
    });
    world.insert_resource(client);
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
