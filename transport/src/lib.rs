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

use bevy_ecs::prelude::Component;

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
