use bevy::prelude::{App, Commands, Event, IntoSystem, Plugin, Res, Resource, Trigger, Update};

use bevy_replicon::core::common_conditions as network_conditions;
use bevy_replicon::prelude::RepliconChannels;
use bevy_replicon_renet2::{
    renet2::{ConnectionConfig, RenetClient},
    RenetChannelsExt, RepliconRenetClientPlugin,
};

pub use bevy_renet2;
pub use bevy_replicon;
pub use bevy_replicon_renet2;

mod transport;

pub struct ClientPlugin {
    pub server_origin: String,
    pub server_port: String,
    pub server_token: String,
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconRenetClientPlugin);

        app.add_plugins(transport::ClientPlugin);
        app.insert_resource(ServerInfo {
            server_origin: self.server_origin.clone(),
            server_port: self.server_port.clone(),
            wt_server_token: self.server_token.clone(),
        });
        app.observe(ConnectToServer::observer)
            .observe(DisconnectFromServer::observer);

        app.add_systems(
            Update,
            network_conditions::client_just_connected.map(|just_connected| {
                if just_connected {
                    bevy::log::debug!("Connected!");
                }
            }),
        );
    }
}

#[derive(Resource)]
pub struct ServerInfo {
    server_origin: String,
    server_port: String,
    wt_server_token: String,
}

#[derive(Event)]
pub struct ConnectToServer;

impl ConnectToServer {
    fn observer(
        _: Trigger<Self>,
        mut commands: Commands,
        channels: Res<RepliconChannels>,
        server_info: Res<ServerInfo>,
    ) {
        let server_channels_config = channels.get_server_configs();
        let client_channels_config = channels.get_client_configs();
        let client = RenetClient::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            ..Default::default()
        });
        commands.insert_resource(client);
        commands.trigger(transport::ConnectToServer::WebTransport {
            server_origin: server_info.server_origin.clone(),
            server_port: server_info.server_port.clone(),
            wt_server_token: server_info.wt_server_token.clone(),
        });
    }
}

#[derive(Event)]
pub struct DisconnectFromServer;

impl DisconnectFromServer {
    fn observer(_: Trigger<Self>, mut commands: Commands) {
        commands.remove_resource::<RenetClient>();
    }
}
