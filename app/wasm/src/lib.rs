#[cfg(not(target_family = "wasm"))]
compile_error!("WASM build must be build for a WASM target.");

use bevy::prelude::*;
use games::{
    // components::Game,
    GameplayPlugin,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    App::default()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1600., 900.).into(),
                canvas: Some("#game-canvas".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }))
        // TODO: reintegrate UI When it's better
        // .configure_sets(
        //     Update,
        //     (
        //         // HomeUISet.run_if(not(any_with_component::<Game>)),
        //         // ChessUISet.run_if(any_with_component::<Game>),
        //     ),
        // )
        .add_plugins((
            GameplayPlugin,
            //  EguiBoardUIPlugin
        ))
        .run();

    Ok(())
}
