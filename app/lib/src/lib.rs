use bevy::prelude::{App, DefaultPlugins, PluginGroup, Window, WindowPlugin};

use wildchess_game::{GameplayPlugin, WildBoardPlugin};
use wildchess_ui::EguiBoardUIPlugin;

pub fn run_app(canvas: Option<String>) {
    App::default()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((GameplayPlugin, WildBoardPlugin, EguiBoardUIPlugin))
        .run();
}
