use bevy::prelude::*;

use bevy_egui::{
    egui::{CentralPanel, Vec2},
    EguiContexts,
};

use games::components::{GameBoard, GameSpawner, WinCondition};
use layouts::{
    ClassicWildLayout, ClassicalLayout, FeaturedWildLayout, KnightRelayLayout, SuperRelayLayout,
};

pub(crate) fn home_menu(mut commands: Commands, mut egui_ctx: EguiContexts) {
    CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
        ui.centered_and_justified(|ui| {
            ui.set_height(300.);
            ui.set_width(200.);
            ui.vertical_centered(|ui| {
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Traditional Chess").clicked() {
                        GameSpawner::new_game(
                            GameBoard::Chess,
                            ClassicalLayout::pieces().into(),
                            WinCondition::RoyalCapture,
                        )
                        .spawn(&mut commands);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Featured Position 1").clicked() {
                        GameSpawner::new_game(
                            GameBoard::Chess,
                            FeaturedWildLayout::One.pieces().into(),
                            WinCondition::RoyalCapture,
                        )
                        .spawn(&mut commands);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Featured Position 2").clicked() {
                        GameSpawner::new_game(
                            GameBoard::Chess,
                            FeaturedWildLayout::Two.pieces().into(),
                            WinCondition::RoyalCapture,
                        )
                        .spawn(&mut commands);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Featured Position 3").clicked() {
                        GameSpawner::new_game(
                            GameBoard::Chess,
                            FeaturedWildLayout::Three.pieces().into(),
                            WinCondition::RoyalCapture,
                        )
                        .spawn(&mut commands);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Wild Chess").clicked() {
                        GameSpawner::new_game(
                            GameBoard::Chess,
                            ClassicWildLayout::pieces().into(),
                            WinCondition::RoyalCapture,
                        )
                        .spawn(&mut commands);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play Super Relay Chess").clicked() {
                        GameSpawner::new_game(
                            GameBoard::Chess,
                            SuperRelayLayout::pieces().into(),
                            WinCondition::RoyalCapture,
                        )
                        .spawn(&mut commands);
                    }
                });
                ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                    if ui.button("Play (Not-Quite) Knight Relay Chess").clicked() {
                        GameSpawner::new_game(
                            GameBoard::Chess,
                            KnightRelayLayout::pieces().into(),
                            WinCondition::RoyalCapture,
                        )
                        .spawn(&mut commands);
                    }
                });
            });
        });
    });
}
