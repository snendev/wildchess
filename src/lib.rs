pub use bevy;
pub use bevy_replicon;

#[cfg(feature = "client")]
pub use client;
pub use games;
pub use layouts;
#[cfg(feature = "server")]
pub use server;
pub use wild_icons;

pub struct WildchessPlugins;

impl bevy::app::PluginGroup for WildchessPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let builder = bevy::app::PluginGroupBuilder::start::<Self>();

        let builder = builder
            .add(games::chess::ChessPlugin)
            .add(games::GameplayPlugin)
            .add(games::MatchmakingPlugin)
            // .add(transport::)
            // TODO: this isn't practically removeable from the PluginGroup.
            // associate with a concept of ActiveGame instead or something?
            // replication may also cause problems here
            .add(wild_icons::PieceIconPlugin::new(get_orientation));

        #[cfg(feature = "client")]
        let builder = builder.add(client::ClientPlugin);
        #[cfg(feature = "server")]
        let builder = builder.add(server::ServerPlugin);

        builder
    }
}

use bevy::prelude::{Query, Res};
use bevy_replicon::prelude::{ClientId, RepliconClient};
use games::{
    chess::{pieces::Orientation, team::Team},
    components::Client,
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
