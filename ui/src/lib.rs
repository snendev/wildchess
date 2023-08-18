use bevy::prelude::{
    not, resource_added, resource_exists, App, Condition, IntoSystemConfigs, Plugin, Update,
};

pub use bevy_egui;

use wildchess_game::GamePieces;

mod icons;
use icons::initialize_icons;
pub use icons::{PieceIcon, PieceIcons};

mod ui;
use ui::{egui_chessboard, read_promotions, IntendedPromotion};

pub struct EguiBoardUIPlugin;

impl Plugin for EguiBoardUIPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_egui::EguiPlugin>() {
            app.add_plugins(bevy_egui::EguiPlugin);
        }
        app.init_resource::<IntendedPromotion>().add_systems(
            Update,
            (
                read_promotions,
                initialize_icons.run_if(
                    resource_added::<GamePieces>().and_then(not(resource_exists::<PieceIcons>())),
                ),
                egui_chessboard.run_if(resource_exists::<PieceIcons>()),
            ),
        );
    }
}
