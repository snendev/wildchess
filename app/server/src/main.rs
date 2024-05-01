#[cfg(target_family = "wasm")]
compile_error!("Native build is not intended for use with WASM. Please build the WASM app.");

use bevy::{
    prelude::{App, Resource},
    MinimalPlugins,
};

use games::GameplayPlugin;
use networking::server::ClientPlugin;

static RUNTIME: std::sync::OnceLock<tokio::runtime::Runtime> = std::sync::OnceLock::new();

#[derive(Clone, Debug)]
#[derive(Resource)]
pub struct TokioHandle(pub tokio::runtime::Handle);

#[tokio::main]
async fn main() {
    let runtime = RUNTIME.get_or_init(|| {
        tokio::runtime::Runtime::new().expect("unable to make default tokio runtime")
    });
    App::default()
        .add_plugins(MinimalPlugins)
        .insert_resource(TokioHandle(runtime.handle().clone()))
        .add_plugins((GameplayPlugin, ClientPlugin))
        .run();
}
