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
            .add(bevy_replicon::prelude::RepliconCorePlugin)
            .add(bevy_replicon::prelude::ParentSyncPlugin)
            .add(games::GameplayPlugin)
            .add(games::MatchmakingPlugin)
            // TODO: this isn't practically removeable from the PluginGroup.
            // associate with a concept of ActiveGame instead or something?
            // replication may also cause problems here
            .add(wild_icons::PieceIconPlugin::new(get_orientation));

        builder
    }
}

#[cfg(feature = "client")]
pub struct WildchessClientPlugins {
    pub server_origin: String,
    pub server_port: String,
    pub server_token: String,
}

#[cfg(feature = "client")]
impl bevy::app::PluginGroup for WildchessClientPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add_group(WildchessPlugins)
            .add(client::ClientPlugin {
                server_origin: self.server_origin,
                server_port: self.server_port,
                server_token: self.server_token,
            })
    }
}

#[cfg(feature = "client")]
impl WildchessPlugins {
    pub fn as_client(
        server_origin: String,
        server_port: String,
        server_token: String,
    ) -> WildchessClientPlugins {
        WildchessClientPlugins {
            server_origin,
            server_port,
            server_token,
        }
    }
}

#[cfg(feature = "server")]
pub struct WildchessServerPlugins {
    pub port: String,
    pub wt_tokens_port: String,
}

#[cfg(feature = "server")]
impl bevy::app::PluginGroup for WildchessServerPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<Self>()
            .add_group(WildchessPlugins)
            .add(server::ServerPlugin {
                port: self.port,
                wt_tokens_port: self.wt_tokens_port,
            })
    }
}

#[cfg(feature = "server")]
impl WildchessPlugins {
    pub fn as_server(port: String, wt_tokens_port: String) -> WildchessServerPlugins {
        WildchessServerPlugins {
            port,
            wt_tokens_port,
        }
    }
}

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
