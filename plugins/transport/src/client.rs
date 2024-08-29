use std::{
    net::{AddrParseError, SocketAddr},
    time::Duration,
};
use thiserror::Error;

use bevy_app::prelude::{App, Plugin};
use bevy_ecs::{event::Event, observer::Trigger, system::Commands};

use renet2::transport::{ClientAuthentication, NetcodeClientTransport, WebServerDestination};

use crate::PROTOCOL_ID;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.observe(ConnectToServer::observer);
    }
}

#[derive(Debug)]
#[derive(Event)]
pub enum ConnectToServer {
    #[cfg(feature = "web_transport_client")]
    WebTransport {
        server_origin: String,
        server_port: String,
        wt_server_token: String,
    },
}

impl ConnectToServer {
    fn observer(trigger: Trigger<Self>, mut commands: Commands) {
        match trigger.event().create_transport() {
            Ok(transport) => {
                commands.insert_resource(transport);
            }
            Err(error) => {
                #[cfg(feature = "log")]
                bevy_log::error!("{error:?}");
            }
        }
    }

    fn create_transport(&self) -> Result<NetcodeClientTransport, Box<dyn std::error::Error>> {
        let current_time = Self::get_current_time()?;
        let client_id: u64 = current_time.as_millis() as u64;
        let socket_addr = self.create_server_address()?;
        #[allow(unused)]
        let authentication = Self::create_authentication(client_id, socket_addr);

        match self {
            #[cfg(feature = "web_transport_client")]
            ConnectToServer::WebTransport {
                wt_server_token, ..
            } => {
                use renet2::transport::WebServerDestination;

                let server_address = WebServerDestination::Addr(socket_addr);

                #[allow(unused)]
                #[allow(clippy::let_unit_value)]
                #[cfg(feature = "web_transport_client")]
                let socket = Self::create_webtransport_socket(server_address, wt_server_token)?;

                #[cfg(all(feature = "web_transport_client", target_family = "wasm"))]
                return Ok(NetcodeClientTransport::new(
                    current_time,
                    authentication,
                    socket,
                )?);
                #[cfg(all(feature = "web_transport_client", not(target_family = "wasm")))]
                return Err(Box::new(WasmCFGError));
            }
        }
    }

    fn create_server_address(&self) -> Result<SocketAddr, AddrParseError> {
        match self {
            #[cfg(feature = "web_transport_client")]
            ConnectToServer::WebTransport {
                server_origin,
                server_port,
                ..
            } => format!("{server_origin}:{server_port}").parse::<SocketAddr>(),
        }
    }

    fn create_authentication(client_id: u64, server_addr: SocketAddr) -> ClientAuthentication {
        ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            socket_id: 0,
            server_addr,
            user_data: None,
        }
    }

    fn get_current_time() -> Result<Duration, SystemTimeError> {
        use wasm_timer::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|_error| SystemTimeError)
    }

    #[cfg(all(feature = "web_transport_client", target_family = "wasm"))]
    fn create_webtransport_socket(
        server_address: WebServerDestination,
        token: &String,
    ) -> Result<renet2::transport::WebTransportClient, ConnectToSocketError> {
        use base64::Engine;
        use renet2::transport::{ServerCertHash, WebTransportClientConfig};

        let hash = base64::engine::general_purpose::STANDARD
            .decode(token)
            .map_err(|error| ConnectToSocketError::WTDecodeHashFailure(error))?;
        let server_cert = ServerCertHash::try_from(hash)
            .map_err(|_error| ConnectToSocketError::WTHashCertFailure(()))?;
        let config =
            WebTransportClientConfig::new_with_certs(server_address, Vec::from([server_cert]));
        Ok(renet2::transport::WebTransportClient::new(config))
    }

    #[allow(unreachable_code)]
    #[cfg(all(feature = "web_transport_client", not(target_family = "wasm")))]
    fn create_webtransport_socket(_: WebServerDestination, _: &String) -> Result<(), WasmCFGError> {
        Err(WasmCFGError)
    }
}

#[derive(Debug, Error)]
pub enum ConnectToSocketError {
    #[cfg(feature = "web_transport_client")]
    #[error("Could not decode hash: {_0:?}")]
    WTDecodeHashFailure(base64::DecodeError),
    #[cfg(feature = "web_transport_client")]
    #[error("Provided token failed to construct a valid cert hash")]
    WTHashCertFailure(()),
}

#[derive(Debug, Error)]
#[error("SystemTime failed to return a current_time")]
pub struct SystemTimeError;

#[derive(Debug, Error)]
#[error("Attempted to create a WebTransport connection but not using wasm. This is unsupported")]
pub struct WasmCFGError;
