use bevy::prelude::*;

use bevy_egui::{
    egui::{CentralPanel, Vec2},
    EguiContexts,
};

use games::components::{GameBoard, GameSpawner, WinCondition};

pub(crate) fn _home_menu(mut commands: Commands, mut egui_ctx: EguiContexts) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.set_height(300.);
            ui.set_width(200.);
            ui.vertical_centered(|ui| {
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Traditional Chess").clicked() {
                        GameSpawner::new_game(GameBoard::Chess, WinCondition::RoyalCapture)
                            .spawn(&mut commands);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Wild Chess").clicked() {
                        GameSpawner::new_game(GameBoard::WildChess, WinCondition::RoyalCapture)
                            .spawn(&mut commands);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Super Relay Chess").clicked() {
                        GameSpawner::new_game(
                            GameBoard::SuperRelayChess,
                            WinCondition::RoyalCapture,
                        )
                        .spawn(&mut commands);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play (Not-Quite) Knight Relay Chess").clicked() {
                        GameSpawner::new_game(
                            GameBoard::KnightRelayChess,
                            WinCondition::RoyalCapture,
                        )
                        .spawn(&mut commands);
                    }
                });
            });
        });
    });
}

// use bevy::prelude::{
//     any_with_component, not, App, DefaultPlugins, IntoSystemConfigs, IntoSystemSetConfigs,
//     PluginGroup, SystemSet, Update, Window, WindowPlugin,
// };

// // use chess_ui::{ChessUISet, EguiBoardUIPlugin};
// use games::{components::Game, GameplayPlugin};

// // mod home_ui;

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
// pub struct HomeUISet;

// pub fn run_app(canvas: Option<String>) {
//     App::default()
//         .add_plugins(DefaultPlugins.set(WindowPlugin {
//             primary_window: Some(Window {
//                 canvas,
//                 resolution: (1600., 900.).into(),
//                 ..Default::default()
//             }),
//             ..Default::default()
//         }))
//         .configure_sets(
//             Update,
//             (
//                 HomeUISet.run_if(not(any_with_component::<Game>)),
//                 // ChessUISet.run_if(any_with_component::<Game>),
//             ),
//         )
//         .add_plugins((
//             GameplayPlugin,
//             //  EguiBoardUIPlugin
//         ))
//         // .add_systems(Update, home_ui::home_menu.in_set(HomeUISet))
//         .run();
// }
