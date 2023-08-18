use bevy_geppetto::Test;

use wildchess_game::{GameplayPlugin, WildBoardPlugin};
use wildchess_ui::EguiBoardUIPlugin;

fn main() {
    Test {
        label: "wild chess".to_string(),
        setup: |app| {
            app.add_plugins(GameplayPlugin)
                .add_plugins(WildBoardPlugin)
                .add_plugins(EguiBoardUIPlugin);
        },
    }
    .run();
}
