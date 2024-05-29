#[cfg(target_family = "wasm")]
compile_error!("Native build is not intended for use with WASM. Please build the WASM app.");

use bevy::{
    app::ScheduleRunnerPlugin,
    log::{Level, LogPlugin},
    prelude::{App, PluginGroup},
    MinimalPlugins,
};

use games::{GameplayPlugin, MatchmakingPlugin};
use replication::ReplicationPlugin;
use transport::server::ServerPlugin as ServerTransportPlugin;

fn main() {
    App::default()
        .add_plugins((
            MinimalPlugins.build().set(ScheduleRunnerPlugin::run_loop(
                // need some wait duration so that async tasks are not entirely outcompeted by the main loop
                std::time::Duration::from_millis(10),
            )),
            LogPlugin {
                filter: "wgpu=error,naga=warn,h3=error".to_string(),
                level: Level::INFO,
                update_subscriber: None,
            },
        ))
        .add_plugins((
            GameplayPlugin,
            MatchmakingPlugin,
            ReplicationPlugin::Server,
            ServerTransportPlugin {
                host: option_env!("SERVER_IP").unwrap_or("0.0.0.0").to_string(),
                wt_port: option_env!("SERVER_PORT").unwrap_or("7636").to_string(),
                wt_tokens_port: option_env!("SERVER_TOKENS_PORT")
                    .unwrap_or("7637")
                    .to_string(),
                // native_host: option_env!("SERVER_IP")
                //     .unwrap_or("127.0.0.1")
                //     .to_string(),
                // native_port: option_env!("SERVER_IP")
                //     .unwrap_or("127.0.0.1")
                //     .to_string(),
            },
        ))
        .run();
}
