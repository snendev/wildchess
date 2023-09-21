use bevy_egui::egui::{Response, Ui, Widget};

struct ClockWidget;

impl Widget for ClockWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.allocate_ui([300., 100.].into(), |ui| {}).response
    }
}
