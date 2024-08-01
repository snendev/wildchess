use bevy::utils::HashMap;

use bevy_egui::egui::{self, Response, Ui, Widget};

use games::chess::board::Square;

use crate::{query::PieceData, widgets::SquareWidget};

pub struct BoardWidget<'a> {
    pieces: &'a HashMap<Square, PieceData<'a>>,
    previous_selection: Option<Square>,
    selection: &'a mut Option<Square>,
}

impl<'a> BoardWidget<'a> {
    pub fn new(
        pieces: &'a HashMap<Square, PieceData<'a>>,
        previous_selection: Option<Square>,
        selection: &'a mut Option<Square>,
    ) -> Self {
        Self {
            selection,
            previous_selection,
            pieces,
        }
    }
}

impl<'a> Widget for BoardWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::Grid::new("board_grid")
            .show(ui, |ui| {
                let selected_piece_data = self
                    .previous_selection
                    .and_then(|square| self.pieces.get(&square));

                for y in (0..=7).rev() {
                    for x in 0..=7 {
                        let square = Square::new(x.try_into().unwrap(), (y).try_into().unwrap());
                        if ui
                            .add(SquareWidget::new_from_context(
                                square,
                                self.pieces.get(&square).and_then(|piece| piece.icon),
                                selected_piece_data,
                            ))
                            .clicked()
                        {
                            *self.selection = Some(square);
                        };
                    }
                    ui.end_row();
                }
            })
            .response
    }
}
