use bevy_app::{Plugin, Startup};
use bevy_ecs::prelude::{Commands, Event, Res, World};

use bevy_replicon::prelude::{ClientPlugin, RepliconChannels, ServerPlugin};
use bevy_replicon_renet2::{
    renet2::{ConnectionConfig, RenetClient, RenetServer},
    RenetChannelsExt, RepliconRenetClientPlugin, RepliconRenetServerPlugin,
};

pub use bevy_replicon::core::common_conditions as network_conditions;

pub enum ReplicationPlugin {
    Server,
    Client,
}

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        match self {
            ReplicationPlugin::Server => {
                app.add_plugins((ServerPlugin::default(), RepliconRenetServerPlugin))
                    .add_systems(Startup, start_server);
            }
            ReplicationPlugin::Client => {
                app.add_plugins((ClientPlugin, RepliconRenetClientPlugin));
                connect_to_server(&mut app.world);
                // app.add_event::<ConnectToServerEvent>()
                //     .add_systems(
                //         Update,
                //         connect_to_server.run_if(on_event::<ConnectToServerEvent>()),
                //     );
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

#[derive(Event)]
pub struct ConnectToServerEvent;

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
