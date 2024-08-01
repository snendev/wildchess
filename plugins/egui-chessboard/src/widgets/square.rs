use bevy_egui::egui::{
    Button, Color32, Image, ImageSource, Response, RichText, Stroke, Ui, Widget,
};
use egui_extras::install_image_loaders;

use games::chess::board::Square;
use wild_icons::PieceIconSvg;

use crate::query::PieceData;

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
            if piece.position.is_some() && piece.position.unwrap().0 == target_square {
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
    icon: Option<&'a PieceIconSvg>,
    highlight: Option<SquareHighlight>,
    // TODO: scale: f32,
}

impl<'a> SquareWidget<'a> {
    pub const WIDTH: f32 = 110.;
    const STROKE_WIDTH: f32 = 4.;
    const DARK_BG: Color32 = Color32::from_rgb(181, 136, 99);
    const LIGHT_BG: Color32 = Color32::from_rgb(240, 217, 181);

    pub fn new_from_context(
        square: Square,
        icon: Option<&'a PieceIconSvg>,
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
            Some(SquareHighlight::Targetable) => Color32::LIGHT_BLUE,
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
        install_image_loaders(ui.ctx());
        let background_color = self.background_color();
        let mut button = match self.icon {
            Some(PieceIconSvg { source, .. }) => {
                let image = ImageSource::Bytes {
                    uri: source.clone().into(),
                    bytes: source.bytes().collect::<Vec<u8>>().into(),
                };
                // TODO: why is this not * 2.?
                const R: f32 = SquareWidget::WIDTH - SquareWidget::STROKE_WIDTH * 3.;
                Button::image_and_text(
                    Image::new(image.clone()).fit_to_exact_size((R, R).into()),
                    "",
                )
                .fill(background_color)
            }
            None => {
                let text = RichText::new("").size(86.).strong().color(Color32::BLACK);
                Button::new(text).fill(background_color)
            } // PieceIcon::Character(character) => {
              //     let text = RichText::new(*character)
              //         .size(86.)
              //         .strong()
              //         .color(Color32::BLACK);
              //     Button::new(text).fill(background_color)
              // }
        };

        if let Some(stroke_color) = self.stroke_color() {
            button = button.stroke(Stroke::new(Self::STROKE_WIDTH, stroke_color));
        }

        ui.add_sized([Self::WIDTH, Self::WIDTH], button)
    }
}
