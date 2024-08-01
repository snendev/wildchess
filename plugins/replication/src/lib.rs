use bevy_replicon_renet2::RepliconRenetPlugins;
use serde::{Deserialize, Serialize};

use bevy_app::prelude::{App, Plugin};
use bevy_ecs::prelude::Component;

use bevy_replicon::prelude::{AppRuleExt, ClientId};

pub use bevy_replicon as replicon;
pub use bevy_replicon_renet2 as replicon_renet2;

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::*;

#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[derive(Component)]
#[derive(Deserialize, Serialize)]
pub struct Client {
    pub id: ClientId,
}

pub struct ReplicationPlugin;

impl Plugin for ReplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RepliconRenetPlugins);

        #[cfg(feature = "server")]
        app.add_plugins(server::ServerReplicationPlugin);
        #[cfg(feature = "client")]
        app.add_plugins(client::ClientReplicationPlugin);

        app.replicate::<Client>();
    }
}
