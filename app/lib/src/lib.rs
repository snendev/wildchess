use bevy::prelude::{App, DefaultPlugins, PluginGroup, Window, WindowPlugin};

use wildchess_game::ChessPlugins;

pub fn run_app(canvas: Option<String>) {
    App::default()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(ChessPlugins)
        .run();
}
