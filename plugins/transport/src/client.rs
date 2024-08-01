use std::net::SocketAddr;
use url::Url;

use bevy_app::prelude::{App, Plugin};
use renet2::transport::WebServerDestination;

use crate::PROTOCOL_ID;

pub struct ClientPlugin {
    pub server_origin: String,
    pub server_port: String,
    #[cfg(feature = "web_transport_client")]
    pub wt_server_token: String,
}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(any(feature = "native_transport", feature = "web_transport_client"))]
        app.add_plugins(NativeClientTransportPlugin::new(
            self.server_origin.as_str(),
            self.server_port.as_str(),
            &self.wt_server_token,
        ));
        #[cfg(feature = "steam_transport")]
        app.add_plugins(SteamClientTransportPlugin);
    }
}

struct NativeClientTransportPlugin {
    server_address: WebServerDestination,
    server_token: String,
}

impl NativeClientTransportPlugin {
    fn new(host: &str, port: &str, server_token: &str) -> Self {
        Self::ip(host, port, server_token)
            // .or_else(|| Self::url(host, port, server_token))
            .unwrap()
    }

    fn url(host: &str, port: &str, server_token: &str) -> Option<Self> {
        format!("{host}:{port}")
            .parse::<Url>()
            .map(|url| Self {
                server_address: WebServerDestination::Url(url),
                server_token: server_token.to_string(),
            })
            .ok()
    }

    fn ip(ip: &str, port: &str, server_token: &str) -> Option<Self> {
        format!("{ip}:{port}")
            .parse::<SocketAddr>()
            .map(|addr| Self {
                server_address: WebServerDestination::Addr(addr),
                server_token: server_token.to_string(),
            })
            .ok()
    }
}

#[cfg(any(feature = "web_transport_client", feature = "native_transport"))]
impl Plugin for NativeClientTransportPlugin {
    fn build(&self, app: &mut App) {
        use renet2::transport::{ClientAuthentication, NetcodeClientTransport};
        #[cfg(not(feature = "web_transport_client"))]
        use std::time::SystemTime;
        #[cfg(feature = "web_transport_client")]
        use wasm_timer::SystemTime;

        app.add_plugins(bevy_renet2::transport::NetcodeClientPlugin);

        let server_addr: SocketAddr = self.server_address.clone().into();
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

            let hash = base64::engine::general_purpose::STANDARD
                .decode(self.server_token.clone())
                .unwrap();
            let config = WebTransportClientConfig::new_with_certs(
                server_addr,
                Vec::from([ServerCertHash::try_from(hash).unwrap()]),
            );
            renet2::transport::WebTransportClient::new(config)
        };

        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

        app.insert_resource(transport);
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
    }
}

#[cfg(feature = "steam_transport")]
impl SteamClientTransportPlugin {
    fn steam_callbacks(client: bevy_ecs::NonSend<steamworks::SingleClient>) {
        client.run_callbacks();
    }
}
