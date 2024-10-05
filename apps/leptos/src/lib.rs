use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use wildchess::{
    bevy::utils::HashMap,
    games::chess::{
        actions::{Action, LastAction},
        board::Square,
        pieces::{Mutation, PieceDefinition, PieceIdentity},
        team::Team,
    },
    games::Clock,
    wild_icons::PieceIconSvg,
};

mod worker;
pub use worker::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: String);
}

pub const SERVER_IP: Option<&str> = option_env!("SERVER_IP");
pub const SERVER_DEFAULT_IP: &str = "127.0.0.1";

pub const SERVER_ORIGIN: Option<&str> = option_env!("SERVER_ORIGIN");
pub const SERVER_DEFAULT_ORIGIN: &str = "http://localhost";

pub const SERVER_PORT: Option<&str> = option_env!("SERVER_PORT");
pub const SERVER_DEFAULT_PORT: &str = "7636";

pub const SERVER_TOKENS_PORT: Option<&str> = option_env!("SERVER_TOKENS_PORT");
pub const SERVER_DEFAULT_TOKENS_PORT: &str = "7637";

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
    State(BoardState),
    Targets(Option<BoardTargets>),
}

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
pub struct BoardState {
    pub size: (u16, u16),
    pub current_turn: Team,
    pub my_team: Team,
    pub pieces: PieceMap,
    pub icons: PieceIconMap,
    pub clocks: Vec<Clock>,
    pub last_action: Option<LastAction>,
}

#[derive(Clone, Debug)]
#[derive(Deserialize, Serialize)]
pub struct BoardTargets {
    pub origin: Square,
    pub actions: HashMap<Square, (Action, Option<PieceDefinition>)>,
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            size: (8, 8),
            current_turn: Default::default(),
            my_team: Default::default(),
            pieces: Default::default(),
            icons: Default::default(),
            clocks: Default::default(),
            last_action: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
pub struct PieceMap(pub HashMap<Square, (PieceIdentity, Team, Option<Mutation>)>);

#[derive(Clone, Debug, Default)]
#[derive(Deserialize, Serialize)]
pub struct PieceIconMap(pub HashMap<PieceIdentity, PieceIconSvg>);
