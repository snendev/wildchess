// #[cfg(not(target_arch = "wasm32"))]
// compile_error!("Compile this for wasm32 only!");

use wasm_bindgen::prelude::*;

use bevy_app::App;
use bevy_ecs::prelude::{Entity, Events};

use games::{
    chess::{
        actions::Actions,
        board::Square,
        pieces::{PieceIdentity, Position},
        team::Team,
    },
    components::{GameBoard, GameSpawner, WinCondition},
    GameplayPlugin, IssueMoveEvent,
};
use wild_icons::PieceIconSvg;

#[wasm_bindgen]
pub struct WasmApp(App);

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct WasmSquare(Square);

#[wasm_bindgen]
impl WasmSquare {
    pub fn get_representation(&self) -> String {
        format!("{}", self.0)
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct WasmPiece(Team, PieceIdentity);

#[wasm_bindgen]
impl WasmPiece {
    #[wasm_bindgen]
    pub fn get_representation(&self) -> String {
        format!(
            "{}{}",
            match self.0 {
                Team::White => 'b',
                Team::Black => 'w',
            },
            match self.1 {
                PieceIdentity::King => "K",
                PieceIdentity::Queen => "Q",
                PieceIdentity::Rook => "R",
                PieceIdentity::Bishop => "B",
                PieceIdentity::Knight => "N",
                PieceIdentity::Pawn => "P",
            }
        )
    }
}

#[wasm_bindgen]
pub struct WasmPiecePosition(WasmPiece, WasmSquare);

#[wasm_bindgen]
impl WasmPiecePosition {
    #[wasm_bindgen]
    pub fn piece(&self) -> WasmPiece {
        self.0
    }

    #[wasm_bindgen]
    pub fn square(&self) -> WasmSquare {
        self.1
    }
}

#[wasm_bindgen]
pub struct WasmIcon {
    piece: WasmPiece,
    svg_source: String,
}

#[wasm_bindgen]
impl WasmIcon {
    // Returns the piece name, like 'wP' for white pawn or 'bN' for black knight.
    #[wasm_bindgen]
    pub fn get_piece(&self) -> String {
        self.piece.get_representation()
    }

    // Returns the icon's svg source string
    #[wasm_bindgen]
    pub fn to_source(self) -> String {
        self.svg_source
    }
}

#[wasm_bindgen]
impl WasmApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmApp {
        let mut app = bevy_app::App::default();
        app.add_plugins(bevy_core::TaskPoolPlugin::default());
        app.add_plugins(bevy_core::TypeRegistrationPlugin);
        app.add_plugins(bevy_core::FrameCountPlugin);
        app.add_plugins(bevy_time::TimePlugin);
        app.add_plugins(bevy_app::ScheduleRunnerPlugin::default());

        app.add_plugins(GameplayPlugin);
        app.add_plugins(wild_icons::PieceIconPlugin);

        WasmApp(app)
    }

    #[wasm_bindgen]
    pub fn start_game(&mut self) {
        // TODO: use blueprints?
        let game_spawner = GameSpawner::new_game(GameBoard::WildChess, WinCondition::RoyalCapture);
        self.0.world.spawn((
            game_spawner.game,
            game_spawner.board,
            game_spawner.win_condition,
        ));
    }

    #[wasm_bindgen]
    pub fn check_game_state(&mut self) -> String {
        let mut query = self.0.world.query::<(&Position, &Team, &PieceIdentity)>();
        let mut buffer = String::from("");
        for (position, team, identity) in query.iter(&self.0.world) {
            buffer.push_str(format!("{:?} {:?}: {:?}\n", team, identity, position).as_str());
        }
        buffer
    }

    #[wasm_bindgen]
    pub fn get_piece_positions(&mut self) -> Vec<WasmPiecePosition> {
        let mut query = self.0.world.query::<(&Position, &Team, &PieceIdentity)>();
        query
            .iter(&self.0.world)
            .map(|(position, team, identity)| {
                WasmPiecePosition(WasmPiece(*team, *identity), WasmSquare(position.0))
            })
            .collect()
    }

    #[wasm_bindgen]
    pub fn get_icons(&mut self) -> Vec<WasmIcon> {
        let mut query = self
            .0
            .world
            .query::<(&PieceIconSvg, &Team, &PieceIdentity)>();
        query
            .iter(&self.0.world)
            .map(|(PieceIconSvg { source, .. }, team, identity)| WasmIcon {
                piece: WasmPiece(*team, *identity),
                svg_source: source.clone(),
            })
            .collect()
    }

    #[wasm_bindgen]
    pub fn get_target_squares(&mut self, square: String) -> Option<Vec<WasmSquare>> {
        let mut query = self.0.world.query::<(&Position, &Actions)>();
        let (_, actions) = query
            .iter(&self.0.world)
            .find(|(position, _)| position.0 == square.as_str().try_into().unwrap())?;
        Some(
            actions
                .0
                .iter()
                .map(|action| WasmSquare(*action.0))
                .collect::<Vec<_>>(),
        )
    }

    #[wasm_bindgen]
    pub fn trigger_move(&mut self, piece_square: String, target_square: String) -> bool {
        // selectedPiece
        let mut query = self.0.world.query::<(Entity, &Position, &Actions)>();
        let Some((piece, _, actions)) = query
            .iter(&self.0.world)
            .find(|(_, position, _)| position.0 == piece_square.as_str().try_into().unwrap())
        else {
            return false;
        };
        let Some((_, action)) = actions
            .0
            .iter()
            .find(|(square, _)| **square == target_square.as_str().try_into().unwrap())
        else {
            return false;
        };
        let action = action.clone();

        let mut move_events = self.0.world.resource_mut::<Events<IssueMoveEvent>>();
        move_events.send(IssueMoveEvent {
            piece,
            action: action.clone(),
        });
        true
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.0.update();
    }
}
