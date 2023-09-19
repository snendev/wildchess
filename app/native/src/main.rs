#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        use chess_app_lib::run_app;

        fn main() {
            run_app(None);
        }
    }
}
