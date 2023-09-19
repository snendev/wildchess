use bevy::prelude::{Commands, Startup};
use bevy_geppetto::Test;

use chess_boards::{BoardPlugin, Game};
use chess_ui::EguiBoardUIPlugin;

use chess_gameplay::GameplayPlugin;

fn main() {
    Test {
        label: "wild chess".to_string(),
        setup: |app| {
            app.add_plugins(GameplayPlugin)
                .add_plugins(BoardPlugin)
                .add_systems(Startup, spawn_game)
                .add_plugins(EguiBoardUIPlugin);
        },
    }
    .run();
}

fn spawn_game(mut commands: Commands) {
    commands.spawn(Game::WildChess);
}
