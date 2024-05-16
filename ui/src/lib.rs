use bevy::prelude::{
    Added, App, Entity, IntoSystemConfigs, Plugin, Query, ResMut, SystemSet, Update,
};

pub use bevy_egui;

use games::{
    chess::pieces::Orientation,
    components::{Game, History},
};
use wild_icons::{PieceIconCharacter, PieceIconPlugin, PieceIconSvg};

pub(crate) mod mutation;
pub(crate) mod query;

mod home_ui;
pub use home_ui::{HomeMenuUIPlugin, HomeMenuUISystems};

mod board_ui;
use board_ui::{
    egui_chessboard, egui_history_panel, egui_information_panel, SelectedGame,
    SelectedHistoricalPly, SelectedSquare,
};

mod widgets;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct ChessUISystems;

pub struct EguiBoardUIPlugin;

impl Plugin for EguiBoardUIPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_egui::EguiPlugin>() {
            app.add_plugins(bevy_egui::EguiPlugin);
        }

        app.add_plugins(PieceIconPlugin::new(get_orientation));

        app.init_resource::<mutation::IntendedMutation>()
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedHistoricalPly>()
            .init_resource::<SelectedGame>()
            .add_systems(
                Update,
                (
                    // mutation::read_mutation_options,
                    History::<PieceIconSvg>::track_component_system,
                    History::<PieceIconCharacter>::track_component_system,
                    set_game,
                    (egui_chessboard, egui_history_panel, egui_information_panel).chain(),
                )
                    .in_set(ChessUISystems),
            );
    }
}

fn set_game(mut game: ResMut<SelectedGame>, game_query: Query<Entity, Added<Game>>) {
    for added_game in game_query.iter() {
        game.0 = Some(added_game);
    }
}

fn get_orientation() -> Orientation {
    Orientation::Up
}
