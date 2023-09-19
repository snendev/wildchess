use bevy::prelude::*;

use bevy_egui::{
    egui::{CentralPanel, Vec2},
    EguiContexts,
};
use chess_boards::Game;

pub(crate) fn home_menu(mut commands: Commands, mut egui_ctx: EguiContexts) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.set_height(300.);
            ui.set_width(200.);
            ui.vertical_centered(|ui| {
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Traditional Chess").clicked() {
                        commands.spawn(Game::Chess);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Wild Chess").clicked() {
                        commands.spawn(Game::WildChess);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Super Relay Chess").clicked() {
                        commands.spawn(Game::SuperRelayChess);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play (Not-Quite) Knight Relay Chess").clicked() {
                        commands.spawn(Game::KnightRelayChess);
                    }
                });
            });
        });
    });
}
