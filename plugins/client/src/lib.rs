use bevy::prelude::{App, Commands, Event, IntoSystem, Plugin, Res, Resource, Trigger, Update};

use bevy_replicon::core::common_conditions as network_conditions;
use bevy_replicon::prelude::RepliconChannels;
use bevy_replicon_renet2::{
    renet2::{ConnectionConfig, RenetClient},
    RenetChannelsExt, RepliconRenetClientPlugin,
};

mod transport;

pub struct ClientPlugin {
    pub server_ip: String,
    pub server_port: String,
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconRenetClientPlugin);
        app.add_plugins(transport::ClientPlugin);
        app.insert_resource(ServerInfo {
            server_ip: self.server_ip.clone(),
            server_port: self.server_port.clone(),
        });
        app.add_observer(ConnectToServer::observer)
            .add_observer(DisconnectFromServer::observer);

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
    server_ip: String,
    server_port: String,
}

#[derive(Event)]
pub struct ConnectToServer {
    pub token: String,
}

impl ConnectToServer {
    fn observer(
        event: Trigger<Self>,
        mut commands: Commands,
        channels: Res<RepliconChannels>,
        server_info: Res<ServerInfo>,
    ) {
        let server_channels_config = channels.get_server_configs();
        let client_channels_config = channels.get_client_configs();
        let client = RenetClient::new(ConnectionConfig {
            server_channels_config,
            client_channels_config,
            available_bytes_per_tick: 60_000,
        });
        commands.insert_resource(client);
        commands.trigger(transport::ConnectToSocket::WebTransport {
            server_ip: server_info.server_ip.clone(),
            server_port: server_info.server_port.clone(),
            wt_server_token: event.event().token.clone(),
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
