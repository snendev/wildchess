use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use wildchess::{
    bevy::utils::HashMap,
    games::chess::{actions::Action, board::Square, pieces::PieceDefinition, team::Team},
    BoardState,
};

mod worker;
pub use worker::*;

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn debug(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn warn(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
pub enum PlayerMessage {
    RequestMove {
        from: Square,
        to: Square,
        promotion_index: Option<usize>,
    },
    SelectPiece {
        square: Square,
    },
    OfferDraw,
    AcceptDraw,
    Resign,
}

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
pub enum WorkerMessage {
    State { state: BoardState, my_team: Team },
    Targets(Option<BoardTargets>),
}

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
pub struct BoardTargets {
    pub origin: Square,
    pub actions: HashMap<Square, (Action, Option<PieceDefinition>)>,
}
