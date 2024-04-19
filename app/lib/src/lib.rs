use bevy::prelude::{
    any_with_component, not, App, DefaultPlugins, IntoSystemConfigs, IntoSystemSetConfigs,
    PluginGroup, SystemSet, Update, Window, WindowPlugin,
};

use chess_ui::{ChessUISet, EguiBoardUIPlugin};
use games::{components::Game, GameplayPlugin};

mod home_ui;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct HomeUISet;

pub fn run_app(canvas: Option<String>) {
    App::default()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas,
                resolution: (1600., 900.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .configure_sets(
            Update,
            (
                HomeUISet.run_if(not(any_with_component::<Game>())),
                ChessUISet.run_if(any_with_component::<Game>()),
            ),
        )
        .add_plugins((GameplayPlugin, EguiBoardUIPlugin))
        .add_systems(Update, home_ui::home_menu.in_set(HomeUISet))
        .run();
}
