use bevy_egui::egui::{Button, Color32, Response, RichText, Stroke, Ui, Vec2, Widget};

use games::chess::board::Square;

use crate::{icons::PieceIcon, query::PieceData};

enum SquareHighlight {
    Selected,
    Targetable,
    CaptureTargetable,
}

impl SquareHighlight {
    pub fn from_context(
        target_square: Square,
        selected_piece: Option<&PieceData<'_>>,
    ) -> Option<Self> {
        if let Some(piece) = selected_piece {
            if piece.position.0 == target_square {
                Some(SquareHighlight::Selected)
            } else if let Some(action) = piece.actions.get(&target_square) {
                if action.captures.is_empty() {
                    Some(SquareHighlight::Targetable)
                } else {
                    Some(SquareHighlight::CaptureTargetable)
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct SquareWidget<'a> {
    square: Square,
    icon: Option<&'a PieceIcon>,
    highlight: Option<SquareHighlight>,
    // TODO: scale: f32,
}

impl<'a> SquareWidget<'a> {
    pub const WIDTH: f32 = 90.;
    const STROKE_WIDTH: f32 = 4.;
    const DARK_BG: Color32 = Color32::from_rgb(181, 136, 99);
    const LIGHT_BG: Color32 = Color32::from_rgb(240, 217, 181);

    pub fn new_from_context(
        square: Square,
        icon: Option<&'a PieceIcon>,
        selected_piece: Option<&'a PieceData<'a>>,
    ) -> Self {
        SquareWidget {
            square,
            icon,
            highlight: SquareHighlight::from_context(square, selected_piece),
        }
    }

    fn background_color(&self) -> Color32 {
        match self.highlight {
            Some(SquareHighlight::Targetable) => Color32::from_rgba_unmultiplied(70, 70, 180, 130),
            Some(SquareHighlight::CaptureTargetable) => {
                Color32::from_rgba_unmultiplied(180, 70, 70, 130)
            }
            _ => {
                if self.square.is_even() {
                    Self::DARK_BG
                } else {
                    Self::LIGHT_BG
                }
            }
        }
    }

    fn stroke_color(&self) -> Option<Color32> {
        match self.highlight {
            Some(SquareHighlight::Selected) => Some(Color32::from_rgb(140, 140, 20)),
            _ => None,
        }
    }
}

impl<'a> Widget for SquareWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let context = ui.ctx();
        let background_color = self.background_color();
        let mut button = match self.icon.unwrap_or(&PieceIcon::Character(' ')) {
            PieceIcon::Svg { image, .. } => {
                // TODO: why is this not * 2.?
                const R: f32 = SquareWidget::WIDTH - SquareWidget::STROKE_WIDTH * 3.;
                Button::image_and_text(image.texture_id(context), Vec2::new(R, R), "")
                    .fill(background_color)
            }
            PieceIcon::Character(character) => {
                let text = RichText::new(*character)
                    .size(64.)
                    .strong()
                    .color(Color32::BLACK);
                Button::new(text).fill(background_color)
            }
        };

        if let Some(stroke_color) = self.stroke_color() {
            button = button.stroke(Stroke::new(Self::STROKE_WIDTH, stroke_color));
        }

        ui.add_sized([Self::WIDTH, Self::WIDTH], button)
    }
}
