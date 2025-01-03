use bevy::app::{PluginGroup, PluginGroupBuilder};

mod replication;
mod transport;

pub struct ServerPlugins {
    pub port: String,
    pub wt_tokens_port: String,
}

impl PluginGroup for ServerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(replication::ServerReplicationPlugin)
            .add(transport::ServerTransportPlugin {
                port: self.port.clone(),
                wt_tokens_port: self.wt_tokens_port.clone(),
            })
    }
}
