// Ensure this is compiled with either client or server enabled
#[cfg(not(any(feature = "client", feature = "server")))]
compile_error!("Must provide features to enable client or server");
// TODO: ultimately this should be supported to represent P2P cases,
// but IDK whether webtransport can handle this
// #[cfg(all(feature = "client", feature = "server"))]
// compile_error!("Cannot compile as both client and server");
// Ensure that at least one of the supported transports is enabled
#[cfg(not(any(
    feature = "native_transport",
    feature = "memory_transport",
    feature = "web_transport_client",
    feature = "web_transport_server",
    feature = "steam_transport"
)))]
compile_error!("Must enable one of the transport features: native (UDP), in-memory, webtransport, and steam transports are supported.");

use serde::{Deserialize, Serialize};
use std::time::Duration;

use bevy_ecs::prelude::{Component, Entity};

use bevy_renet2::renet2::{ChannelConfig, ConnectionConfig, SendType};
use bevy_replicon::prelude::ClientId;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;

pub const PRIVATE_KEY: &[u8; bevy_renet2::renet2::transport::NETCODE_KEY_BYTES] =
    b"an example very very secret key."; // 32-bytes
pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug)]
#[derive(Component)]
pub struct Player {
    pub id: ClientId,
}

pub enum ClientChannel {
    Command,
}

pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkedEntities {
    pub entities: Vec<Entity>,
    // pub translations: Vec<[f32; 3]>,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Command => 0,
        }
    }
}

impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![ChannelConfig {
            channel_id: Self::Command.into(),
            max_memory_usage_bytes: 5 * 1024 * 1024,
            send_type: SendType::ReliableOrdered {
                resend_time: Duration::ZERO,
            },
        }]
    }
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::NetworkedEntities => 0,
            ServerChannel::ServerMessages => 1,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ChannelConfig {
                channel_id: Self::NetworkedEntities.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::Unreliable,
            },
            ChannelConfig {
                channel_id: Self::ServerMessages.into(),
                max_memory_usage_bytes: 10 * 1024 * 1024,
                send_type: SendType::ReliableOrdered {
                    resend_time: Duration::from_millis(200),
                },
            },
        ]
    }
}

pub(crate) fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: ClientChannel::channels_config(),
        server_channels_config: ServerChannel::channels_config(),
    }
}