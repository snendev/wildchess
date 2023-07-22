use bevy_geppetto::Test;

use wildchess_game::ChessPlugins;

fn main() {
    Test {
        label: "wild chess".to_string(),
        setup: |app| {
            app.add_plugins(ChessPlugins);
        },
    }
    .run();
}
