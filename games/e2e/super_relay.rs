use bevy::prelude::{Commands, Startup};

use bevy_geppetto::Test;
use chess_ui::EguiBoardUIPlugin;
use games::{
    components::{GameBoard, GameSpawner, WinCondition},
    GameplayPlugin,
};

fn main() {
    Test {
        label: "test super relay board".to_string(),
        setup: |app| {
            app.add_plugins((GameplayPlugin, EguiBoardUIPlugin))
                .add_systems(Startup, spawn_game);
        },
    }
    .run()
}

fn spawn_game(mut commands: Commands) {
    GameSpawner::new_game(GameBoard::SuperRelayChess, WinCondition::RoyalCapture)
        .spawn(&mut commands);
}
