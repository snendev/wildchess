use bevy_geppetto::Test;

use wildchess_game::{BoardPlugin, GameplayPlugin};
use wildchess_ui::EguiBoardUIPlugin;

fn main() {
    Test {
        label: "wild chess".to_string(),
        setup: |app| {
            app.add_plugins(GameplayPlugin)
                .add_plugins(BoardPlugin)
                .add_plugins(EguiBoardUIPlugin);
        },
    }
    .run();
}
