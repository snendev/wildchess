use bevy::prelude::{
    Added, App, Entity, IntoSystemConfigs, Plugin, Query, ResMut, SystemSet, Update,
};

pub use bevy_egui;

mod widgets;

mod icons;
use games::components::{Game, History};
pub use icons::PieceIcon;

pub(crate) mod mutation;
pub(crate) mod query;

mod board_ui;
use board_ui::{
    egui_chessboard, egui_history_panel, egui_information_panel, SelectedGame,
    SelectedHistoricalPly, SelectedSquare,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct ChessUISet;

pub struct EguiBoardUIPlugin;

impl Plugin for EguiBoardUIPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_egui::EguiPlugin>() {
            app.add_plugins(bevy_egui::EguiPlugin);
        }
        app.init_resource::<mutation::IntendedMutation>()
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedHistoricalPly>()
            .init_resource::<SelectedGame>()
            .add_systems(
                Update,
                (
                    mutation::read_mutation_options,
                    PieceIcon::attach_icons_system,
                    History::<PieceIcon>::track_component_system,
                    set_game,
                    (egui_chessboard, egui_history_panel, egui_information_panel).chain(),
                )
                    .in_set(ChessUISet),
            );
    }
}

fn set_game(mut game: ResMut<SelectedGame>, game_query: Query<Entity, Added<Game>>) {
    for added_game in game_query.iter() {
        game.0 = Some(added_game);
    }
}
