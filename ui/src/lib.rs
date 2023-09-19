use bevy::prelude::{App, Plugin, Update};

pub use bevy_egui;

mod icons;
pub use icons::PieceIcon;

pub(crate) mod mutation;
pub(crate) mod query;

mod board_ui;
use board_ui::egui_chessboard;

pub struct EguiBoardUIPlugin;

impl Plugin for EguiBoardUIPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_egui::EguiPlugin>() {
            app.add_plugins(bevy_egui::EguiPlugin);
        }
        app.init_resource::<mutation::IntendedMutation>()
            .add_systems(
                Update,
                (
                    mutation::read_mutation_options,
                    icons::attach_piece_icons,
                    egui_chessboard,
                ),
            );
    }
}