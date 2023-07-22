use bevy::{
    app::PluginGroupBuilder,
    prelude::{App, IntoSystemConfigs, Plugin, PluginGroup, Startup, Update},
};

use bevy_egui::EguiPlugin;

use crate::{board::wild_board, MovePieceEvent};

mod systems;
use systems::{calculate_piece_vision, capture_pieces, move_pieces};

mod ui;
use ui::egui_chessboard;

pub struct WildBoardPlugin;

impl Plugin for WildBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, wild_board);
    }
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovePieceEvent>().add_systems(
            Update,
            (move_pieces, capture_pieces, calculate_piece_vision).chain(),
        );
    }
}

pub struct EguiBoardUIPlugin;

impl Plugin for EguiBoardUIPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
        app.add_systems(Update, egui_chessboard);
    }
}

pub struct ChessPlugins;

impl PluginGroup for ChessPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        group = group
            .add(WildBoardPlugin)
            .add(GameplayPlugin)
            .add(EguiBoardUIPlugin);
        group
    }
}
