#[cfg(target_family = "wasm")]
compile_error!("Native build is not intended for use with WASM. Please build the WASM app.");

use bevy::{
    prelude::{App, Resource},
    MinimalPlugins,
};

use games::GameplayPlugin;
use networking::server::ServerPlugin;

#[derive(Clone, Debug)]
#[derive(Resource)]
pub struct TokioHandle(pub tokio::runtime::Handle);

#[tokio::main]
async fn main() {
    App::default()
        .add_plugins(MinimalPlugins)
        .insert_resource(TokioHandle(tokio::runtime::Handle::try_current().unwrap()))
        .add_plugins((GameplayPlugin, ServerPlugin))
        .run();
}
