use bevy::prelude::{App, Plugin, Update};

pub use bevy_egui;

mod icons;
pub use icons::PieceIcon;

pub(crate) mod promotion;

mod ui;
use ui::egui_chessboard;

pub struct EguiBoardUIPlugin;

impl Plugin for EguiBoardUIPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_egui::EguiPlugin>() {
            app.add_plugins(bevy_egui::EguiPlugin);
        }
        app.init_resource::<promotion::IntendedPromotion>()
            .add_systems(
                Update,
                (
                    promotion::read_promotions,
                    icons::attach_piece_icons,
                    egui_chessboard,
                ),
            );
    }
}
