use bevy_geppetto::Test;

use wild_board::BoardPlugin;
use wildchess_ui::EguiBoardUIPlugin;

use chess_gameplay::GameplayPlugin;

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
