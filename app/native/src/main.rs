#[cfg(target_family = "wasm")]
compile_error!("Native build is not intended for use with WASM. Please build the WASM app.");

use bevy::prelude::{
    any_with_component, not, App, DefaultPlugins, IntoSystemConfigs, IntoSystemSetConfigs,
    PluginGroup, SystemSet, Update, Window, WindowPlugin,
};

use chess_ui::{ChessUISystems, EguiBoardUIPlugin, HomeMenuUIPlugin, HomeMenuUISystems};
use games::{components::Game, GameplayPlugin};

fn main() {
    App::default()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1600., 900.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .configure_sets(
            Update,
            (
                HomeMenuUISystems.run_if(not(any_with_component::<Game>)),
                ChessUISystems.run_if(any_with_component::<Game>),
            ),
        )
        .add_plugins((GameplayPlugin, HomeMenuUIPlugin, EguiBoardUIPlugin))
        .run();
}
