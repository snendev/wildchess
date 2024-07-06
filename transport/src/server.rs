use std::net::SocketAddr;

#[cfg(feature = "web_transport_server")]
use warp::Filter;

use bevy_app::prelude::{App, Plugin};
use bevy_ecs::prelude::Resource;

use crate::PROTOCOL_ID;

pub struct ServerPlugin {
    #[cfg(feature = "web_transport_server")]
    pub host: String,
    #[cfg(feature = "web_transport_server")]
    pub wt_port: String,
    #[cfg(feature = "web_transport_server")]
    pub wt_tokens_port: String,
    #[cfg(feature = "native_transport")]
    pub native_host: String,
    #[cfg(feature = "native_transport")]
    pub native_port: String,
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(any(feature = "web_transport_server", feature = "native_transport"))]
        app.add_plugins(NativeServerTransportPlugin {
            host: self.host.clone(),
            port: self.wt_port.clone(),
            tokens_port: self.wt_tokens_port.clone(),
        });

        #[cfg(feature = "steam_transport")]
        app.add_plugins(SteamServerTransportPlugin);
    }
}

#[cfg(any(feature = "web_transport_server", feature = "native_transport"))]
struct NativeServerTransportPlugin {
    host: String,
    port: String,
    tokens_port: String,
}

#[cfg(any(feature = "web_transport_server", feature = "native_transport"))]
impl NativeServerTransportPlugin {
    fn new(host: String, port: String, tokens_port: String) -> Self {
        Self {
            host,
            port,
            tokens_port,
        }
    }
}

#[cfg(any(feature = "web_transport_server", feature = "native_transport"))]
impl Default for NativeServerTransportPlugin {
    fn default() -> Self {
        Self::new(
            "0.0.0.0".to_string(),
            "7636".to_string(),
            "7637".to_string(),
        )
    }
}

#[cfg(any(feature = "web_transport_server", feature = "native_transport"))]
impl Plugin for NativeServerTransportPlugin {
    fn build(&self, app: &mut App) {
        use bevy_renet2::renet2::transport::{
            NetcodeServerTransport, ServerAuthentication, ServerSetupConfig,
        };
        use bevy_renet2::transport::NetcodeServerPlugin;
        use std::time::SystemTime;

        app.add_plugins(NetcodeServerPlugin);

        let public_addr = format!("{}:{}", self.host, self.port).parse().unwrap();

        let current_time: std::time::Duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let server_config = ServerSetupConfig {
            current_time,
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            socket_addresses: vec![vec![public_addr]],
            authentication: ServerAuthentication::Unsecure,
        };

        #[cfg(feature = "native_transport")]
        let socket = {
            let udp_socket = std::net::UdpSocket::bind(public_addr).unwrap();
            renet2::transport::NativeSocket::new(udp_socket).unwrap()
        };

        #[cfg(feature = "web_transport_server")]
        let socket = {
            use base64::Engine;

            #[derive(Resource)]
            pub struct TokioRuntime(#[allow(dead_code)] tokio::runtime::Runtime);

            println!("Opening WT Socket on {}", public_addr);

            let (config, cert_hash) =
                renet2::transport::WebTransportServerConfig::new_selfsigned(public_addr, 4);

            let cert_hash_b64 =
                base64::engine::general_purpose::STANDARD.encode(cert_hash.hash.as_ref());

            let runtime = tokio::runtime::Runtime::new().unwrap();

            let certs_socket: SocketAddr = format!("{}:{}", self.host, self.tokens_port)
                .parse()
                .unwrap();
            runtime.spawn(async move {
                let cors = warp::cors()
                    .allow_method("GET")
                    .allow_origin("http://localhost:8000")
                    .allow_origin("http://127.0.0.1:8000")
                    .allow_origin("https://wildchess.dev")
                    .allow_origin("https://wildchess.deno.dev")
                    .allow_origin("https://www.wildchess.dev");
                let serve_certs = warp::path::end()
                    .map(move || cert_hash_b64.clone())
                    .with(cors);
                warp::serve(serve_certs).run(certs_socket).await;
            });

            let socket =
                renet2::transport::WebTransportServer::new(config, runtime.handle().clone())
                    .unwrap();
            app.insert_resource(TokioRuntime(runtime));
            socket
        };

        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
        app.insert_resource(transport);
    }
}

// TODO: untested
#[cfg(feature = "steam_transport")]
struct SteamServerTransportPlugin;

#[cfg(feature = "steam_transport")]
impl Plugin for SteamServerTransportPlugin {
    fn build(&self, app: &mut App) {
        use bevy_app::PreUpdate;
        use renet2_steam::bevy::{SteamServerConfig, SteamServerPlugin, SteamServerTransport};
        use renet2_steam::AccessPermission;

        let (steam_client, single) = steamworks::Client::init_app(480).unwrap();

        let server: RenetServer = RenetServer::new(connection_config());

        let steam_transport_config = SteamServerConfig {
            max_clients: 10,
            access_permission: AccessPermission::Public,
        };
        let transport = SteamServerTransport::new(&steam_client, steam_transport_config).unwrap();

        app.add_plugins(SteamServerPlugin);
        app.insert_resource(server);
        app.insert_non_send_resource(transport);
        app.insert_non_send_resource(single);

        app.add_systems(PreUpdate, Self::steam_callbacks);
    }
}

#[cfg(feature = "steam_transport")]
impl SteamServerTransportPlugin {
    fn steam_callbacks(client: bevy_ecs::prelude::NonSend<steamworks::SingleClient>) {
        client.run_callbacks();
    }
}
