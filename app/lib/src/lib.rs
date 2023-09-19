use bevy::prelude::{
    any_with_component, not, App, DefaultPlugins, IntoSystemConfigs, IntoSystemSetConfig,
    PluginGroup, SystemSet, Update, Window, WindowPlugin,
};

use chess_boards::{BoardPlugin, Game};
use chess_gameplay::GameplayPlugin;
use chess_ui::{ChessUISet, EguiBoardUIPlugin};

mod home_ui;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct HomeUISet;

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
        .configure_set(Update, HomeUISet.run_if(not(any_with_component::<Game>())))
        .configure_set(Update, ChessUISet.run_if(any_with_component::<Game>()))
        .add_plugins((GameplayPlugin, BoardPlugin, EguiBoardUIPlugin))
        .add_systems(Update, home_ui::home_menu.in_set(HomeUISet))
        .run();
}
