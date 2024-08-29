use bevy::prelude::*;

use bevy_egui::{
    egui::{CentralPanel, Vec2},
    EguiContexts,
};

use games::components::SpawnGame;
use layouts::*;

pub struct HomeMenuUIPlugin;

impl Plugin for HomeMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::menu_system.in_set(HomeMenuUISystems));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, SystemSet)]
pub struct HomeMenuUISystems;

impl HomeMenuUIPlugin {
    pub fn menu_system(mut commands: Commands, mut egui_ctx: EguiContexts) {
        CentralPanel::default().show(egui_ctx.ctx_mut(), |ui| {
            ui.centered_and_justified(|ui| {
                ui.set_height(300.);
                ui.set_width(200.);
                ui.vertical_centered(|ui| {
                    ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                        if ui.button("Play Traditional Chess").clicked() {
                            commands.trigger(SpawnGame::new(ClassicalLayout::pieces().into()));
                        }
                    });
                    ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                        if ui.button("Play Wild Chess").clicked() {
                            commands.trigger(SpawnGame::new(ClassicWildLayout::pieces().into()));
                        }
                    });
                    ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                        if ui.button("Play Super Relay Chess").clicked() {
                            commands.trigger(SpawnGame::new(SuperRelayLayout::pieces().into()));
                        }
                    });
                    ui.allocate_ui(Vec2::new(250., 120.), |ui| {
                        if ui.button("Play (Not-Quite) Knight Relay Chess").clicked() {
                            commands.trigger(SpawnGame::new(KnightRelayLayout::pieces().into()));
                        }
                    });
                });
            });
        });
    }
}
