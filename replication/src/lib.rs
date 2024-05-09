use serde::{Deserialize, Serialize};

use bevy_app::Plugin;
use bevy_ecs::prelude::{Component, Entity, Event};

use bevy_replicon::{
    core::{replication_rules::AppReplicationExt, replicon_channels::ChannelKind},
    network_event::client_event::ClientEventAppExt,
    prelude::ClientId,
};

pub struct ReplicationPlugin;

// impl Plugin for ReplicationPlugin {
//     fn build(&self, app: &mut bevy_app::App) {
//         app.replicate()
//             .replicate()
//             .replicate()
//             .replicate()
//             .add_client_event::<PlayerCommand>(ChannelKind::Ordered);
//     }
// }
