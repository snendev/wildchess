#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        use bevy::prelude::{
            any_with_component, not, App, DefaultPlugins, IntoSystemConfigs, IntoSystemSetConfigs,
            PluginGroup, SystemSet, Update, Window, WindowPlugin,
        };

        // use chess_ui::{ChessUISet, EguiBoardUIPlugin};
        use games::{components::Game, GameplayPlugin};

        fn main() {
            App::default().add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1600., 900.).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }))
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
        }
    }
}
