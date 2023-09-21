use std::time::Duration;

use bevy::prelude::{Commands, Startup};

use bevy_geppetto::Test;
use chess_ui::EguiBoardUIPlugin;
use games::{
    components::{Clock, GameBoard, GameSpawner, WinCondition},
    GameplayPlugin,
};

fn main() {
    Test {
        label: "test clocks".to_string(),
        setup: |app| {
            app.add_plugins((GameplayPlugin, EguiBoardUIPlugin))
                .add_systems(Startup, spawn_game);
        },
    }
    .run()
}

fn spawn_game(mut commands: Commands) {
    GameSpawner::new_game(GameBoard::Chess, WinCondition::RoyalCapture)
        .with_clock(Clock::new(
            Duration::from_secs(3 * 60),
            Duration::from_secs(1),
        ))
        .spawn(&mut commands);
}
