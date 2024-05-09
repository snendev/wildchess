// #[cfg(not(target_arch = "wasm32"))]
// compile_error!("Compile this for wasm32 only!");

use crossbeam::channel::{unbounded, Receiver, Sender};
use wasm_bindgen::prelude::*;

use bevy_app::{App, PreUpdate};
use bevy_ecs::prelude::{Entity, Events, Res, Resource, With};

use bevy_replicon::prelude::ClientPlugin as RepliconClientPlugin;
use bevy_replicon_renet2::RepliconRenetClientPlugin;

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
use transport::client::ClientPlugin as ClientTransportPlugin;
use wild_icons::PieceIconSvg;

// Use this to enable console logging
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

#[wasm_bindgen]
pub struct WasmApp(App);

#[wasm_bindgen]
impl WasmApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmApp {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let (tx, rx) = unbounded::<Ping>();
        CHANNEL
            .set(tx)
            .expect("to be able to create a crossbeam channel");

        let mut app = bevy_app::App::default();
        app.add_plugins((
            bevy_core::TaskPoolPlugin::default(),
            bevy_core::TypeRegistrationPlugin,
            bevy_core::FrameCountPlugin,
            bevy_time::TimePlugin,
            bevy_app::ScheduleRunnerPlugin::default(),
        ));
        app.add_plugins((
            GameplayPlugin,
            RepliconClientPlugin,
            RepliconRenetClientPlugin,
            ClientTransportPlugin,
        ));
        app.add_plugins(wild_icons::PieceIconPlugin);

        app.insert_resource(PingReceiver(rx));
        app.add_systems(PreUpdate, PingReceiver::receive_message_system);

        WasmApp(app)
    }

    #[wasm_bindgen]
    pub fn setup_board(&mut self) {
        let game_spawner = GameSpawner::new_game(GameBoard::WildChess, WinCondition::RoyalCapture);
        self.0.world.spawn((
            game_spawner.game,
            game_spawner.board,
            game_spawner.win_condition,
        ));
    }

    #[wasm_bindgen]
    pub fn remove_board(&mut self) {
        let mut game_query = self.0.world.query_filtered::<Entity, With<GameBoard>>();
        for entity in game_query.iter(&self.0.world).collect::<Vec<_>>() {
            self.0.world.entity_mut(entity).despawn();
        }
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

    #[wasm_bindgen]
    pub fn run(&mut self) {
        self.0.run();
    }
}

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
                Team::White => 'w',
                Team::Black => 'b',
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

struct Ping;

static CHANNEL: std::sync::OnceLock<Sender<Ping>> = std::sync::OnceLock::new();

#[wasm_bindgen]
pub struct PingSender(Sender<Ping>);

#[wasm_bindgen]
impl PingSender {
    pub fn get() -> Self {
        PingSender(CHANNEL.get().expect("channel to be initialized").clone())
    }

    #[wasm_bindgen]
    pub fn send(&self) {
        // self.
    }
}

#[derive(Resource)]
pub struct PingReceiver(Receiver<Ping>);

impl PingReceiver {
    fn receive_message_system(receiver: Res<PingReceiver>) {}
}
