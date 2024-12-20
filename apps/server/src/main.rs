#[cfg(target_family = "wasm")]
compile_error!("Native build is not intended for use with WASM. Please build the WASM app.");

use bevy::{
    app::{App, PluginGroup, ScheduleRunnerPlugin},
    log::{Level, LogPlugin},
    MinimalPlugins,
};

use wildchess::WildchessPlugins;

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
                ..Default::default()
            },
            WildchessPlugins::as_server(
                option_env!("SERVER_PORT").unwrap_or("7636").to_string(),
                option_env!("SERVER_TOKENS_PORT")
                    .unwrap_or("7637")
                    .to_string(),
            ),
        ))
        .run();
}
