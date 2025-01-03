pub use bevy;
pub use bevy_replicon;

#[cfg(feature = "client")]
pub use client;
pub use games;
pub use layouts;
#[cfg(feature = "server")]
pub use server;
pub use wild_icons;

mod active_game;
pub use active_game::*;

mod board_state;
pub use board_state::*;

pub struct WildchessPlugins;

impl bevy::app::PluginGroup for WildchessPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let builder = bevy::app::PluginGroupBuilder::start::<Self>();

        let builder = builder
            .add_group(bevy_replicon::prelude::RepliconPlugins)
            .add(games::GameplayPlugin)
            .add(games::MatchmakingPlugin)
            // TODO: this isn't practically removeable from the PluginGroup.
            // associate with a concept of ActiveGame instead or something?
            // replication may also cause problems here
            .add(wild_icons::PieceIconPlugin::new(get_orientation));

        #[cfg(feature = "client")]
        let builder = builder.add(client::ClientPlugin {
            server_ip: SERVER_IP.unwrap_or(SERVER_DEFAULT_IP).to_string(),
            server_port: SERVER_PORT.unwrap_or(SERVER_DEFAULT_PORT).to_string(),
        });
        #[cfg(feature = "server")]
        let builder = builder
            .set(bevy_replicon::server::ServerPlugin {
                visibility_policy: bevy_replicon::prelude::VisibilityPolicy::Whitelist,
                ..Default::default()
            })
            .add_group(server::ServerPlugins {
                port: SERVER_PORT.unwrap_or(SERVER_DEFAULT_PORT).to_string(),
                wt_tokens_port: SERVER_TOKENS_PORT
                    .unwrap_or(SERVER_DEFAULT_TOKENS_PORT)
                    .to_string(),
            });

        builder
    }
}

#[cfg(any(feature = "client", feature = "server"))]
pub mod network_constants {
    pub const SERVER_IP: Option<&str> = option_env!("SERVER_IP");
    pub const SERVER_DEFAULT_IP: &str = "127.0.0.1";

    pub const SERVER_ORIGIN: Option<&str> = option_env!("SERVER_ORIGIN");
    pub const SERVER_DEFAULT_ORIGIN: &str = "http://localhost";

    pub const SERVER_PORT: Option<&str> = option_env!("SERVER_PORT");
    pub const SERVER_DEFAULT_PORT: &str = "7636";

    pub const SERVER_TOKENS_PORT: Option<&str> = option_env!("SERVER_TOKENS_PORT");
    pub const SERVER_DEFAULT_TOKENS_PORT: &str = "7637";
}
#[cfg(any(feature = "client", feature = "server"))]
use network_constants::*;

use bevy::prelude::{Query, Res};
use bevy_replicon::prelude::{ClientId, RepliconClient};
use games::{
    chess::{pieces::Orientation, team::Team},
    components::Client,
    GameSystems,
};

fn get_orientation(
    client: Option<Res<RepliconClient>>,
    players: Query<(&Team, Option<&Client>)>,
) -> Orientation {
    if let Some((team, _)) = client.and_then(|client| client.id()).and_then(|client_id| {
        players.iter().find(|(_, player)| {
            player.map(|client| client.id).unwrap_or(ClientId::SERVER) == client_id
        })
    }) {
        team.orientation()
    } else {
        Orientation::Up
    }
}

pub struct WildchessIntegrationPlugin;

impl bevy::app::Plugin for WildchessIntegrationPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        use bevy::prelude::IntoSystemConfigs;
        app.add_systems(
            bevy::app::Update,
            (
                active_game::Active::set_active,
                board_state::BoardState::track_state,
            )
                .chain()
                .after(GameSystems::All),
        );
    }
}
