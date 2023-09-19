use bevy::prelude::{Commands, Startup};

use bevy_geppetto::Test;
use chess_boards::{BoardPlugin, Game};
use chess_gameplay::GameplayPlugin;
use chess_ui::EguiBoardUIPlugin;

fn main() {
    Test {
        label: "test classical board".to_string(),
        setup: |app| {
            app.add_plugins((GameplayPlugin, EguiBoardUIPlugin, BoardPlugin))
                .add_systems(Startup, spawn_game);
        },
    }
    .run()
}

fn spawn_game(mut commands: Commands) {
    commands.spawn(Game::SuperRelayChess);
}
