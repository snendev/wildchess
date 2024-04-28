#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;
        use bevy::prelude::*;
        use games::{components::Game, GameplayPlugin};

        #[wasm_bindgen(start)]
        pub fn main() -> Result<(), JsValue> {
            App::default().add_plugins(DefaultPlugins.set(WindowPlugin {
                canvas: Some("#game-canvas".to_string()),
                primary_window: Some(Window {
                    resolution: (1600., 900.).into(),
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
    }
}
