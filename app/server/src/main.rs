#[cfg(target_family = "wasm")]
compile_error!("Native build is not intended for use with WASM. Please build the WASM app.");

use bevy::{
    app::ScheduleRunnerPlugin,
    log::{Level, LogPlugin},
    prelude::{App, PluginGroup},
    MinimalPlugins,
};

use games::GameplayPlugin;
use networking::server::ServerPlugin;

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
        .add_plugins((GameplayPlugin, ServerPlugin))
        .run();
}
