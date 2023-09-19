use bevy::prelude::{App, DefaultPlugins, PluginGroup, Window, WindowPlugin};

use chess_boards::BoardPlugin;
use chess_gameplay::GameplayPlugin;
use chess_ui::EguiBoardUIPlugin;

pub fn run_app(canvas: Option<String>) {
    App::default()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas,
                resolution: (1366., 768.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((GameplayPlugin, BoardPlugin, EguiBoardUIPlugin))
        .run();
}
