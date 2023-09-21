use bevy::prelude::{App, IntoSystemConfigs, Plugin, SystemSet, Update};

pub use bevy_egui;

mod widgets;

mod icons;
pub use icons::PieceIcon;

pub(crate) mod mutation;
pub(crate) mod query;

mod board_ui;
use board_ui::egui_chessboard;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct ChessUISet;

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
                )
                    .in_set(ChessUISet),
            );
    }
}
