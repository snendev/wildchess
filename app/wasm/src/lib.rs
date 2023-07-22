#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;
        use wildchess_app_lib::run_app;

        #[wasm_bindgen(start)]
        pub fn main() -> Result<(), JsValue> {
            run_app(Some("#game-canvas".to_string()));

            Ok(())
        }
    }
}
