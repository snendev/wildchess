use bevy_egui::egui::{Button, Color32, Response, RichText, Stroke, Ui, Widget};
use games::components::Clock;

pub struct ClockWidget<'a> {
    clock: &'a Clock,
}

impl<'a> ClockWidget<'a> {
    const WIDTH: f32 = 200.;
    const HEIGHT: f32 = 100.;
    const FONT_SIZE: f32 = 64.;
    const BG_COLOR: Color32 = Color32::LIGHT_GRAY;
    const STROKE_WIDTH: f32 = 4.;
    const STROKE_COLOR: Color32 = Color32::BLACK;

    pub fn new(clock: &'a Clock) -> Self {
        ClockWidget { clock }
    }
}

impl<'a> Widget for ClockWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let remaining_seconds = self.clock.remaining_seconds();
        let minutes = remaining_seconds / 60;
        let seconds = remaining_seconds % 60;
        let text = RichText::new(format!("{}:{:0>2}", minutes, seconds))
            .size(Self::FONT_SIZE)
            .strong()
            .color(Color32::BLACK);

        let button = Button::new(text)
            .fill(Self::BG_COLOR)
            .stroke(Stroke::new(Self::STROKE_WIDTH, Self::STROKE_COLOR));
        ui.add_sized([Self::WIDTH, Self::HEIGHT], button)
    }
}
