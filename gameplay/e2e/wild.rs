use bevy::prelude::{EventWriter, Startup};
use bevy_geppetto::Test;

use chess_boards::{BoardPlugin, BuildWildBoardEvent};
use wildchess_ui::EguiBoardUIPlugin;

use chess_gameplay::GameplayPlugin;

fn main() {
    Test {
        label: "wild chess".to_string(),
        setup: |app| {
            app.add_plugins(GameplayPlugin)
                .add_plugins(BoardPlugin)
                .add_systems(Startup, spawn_board)
                .add_plugins(EguiBoardUIPlugin);
        },
    }
    .run();
}

fn spawn_board(mut writer: EventWriter<BuildWildBoardEvent>) {
    writer.send(BuildWildBoardEvent);
}
