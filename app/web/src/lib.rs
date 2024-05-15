// #[cfg(not(target_arch = "wasm32"))]
// compile_error!("Compile this for wasm32 only!");

use wasm_bindgen::prelude::*;

use bevy_app::App;
use bevy_ecs::{
    prelude::{Entity, Events, Query, Res, With},
    system::RunSystemOnce,
};

use games::{
    chess::{
        actions::Actions,
        board::Square,
        pieces::{Orientation, PieceIdentity, Position},
        team::Team,
    },
    components::{Game, GameBoard, GameRequestClock, GameRequestVariant, LastMove},
    GameOpponent, GameplayPlugin, MatchmakingPlugin, RequestJoinGameEvent, RequestTurnEvent,
};
use replication::{
    bevy_replicon::{core::common_conditions as network_conditions, prelude::RepliconClient},
    ConnectToServerEvent, Player, ReplicationPlugin,
};
use transport::client::ClientPlugin as ClientTransportPlugin;
use wild_icons::PieceIconSvg;

// Use this to enable console logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct WasmApp(App);

#[wasm_bindgen]
impl WasmApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmApp {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

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
            MatchmakingPlugin,
            ReplicationPlugin::Client,
            ClientTransportPlugin,
        ));
        app.add_plugins(wild_icons::PieceIconPlugin::new(get_orientation));
        app.world.send_event(ConnectToServerEvent);

        WasmApp(app)
    }

    #[wasm_bindgen]
    pub fn request_game(&mut self, game_request: WasmGameRequest) {
        self.0.world.send_event(RequestJoinGameEvent {
            opponent: GameOpponent::Online,
            game: game_request.variant,
            clock: game_request.clock,
        });
    }

    #[wasm_bindgen]
    pub fn is_connected(&mut self) -> bool {
        self.0
            .world
            .run_system_once(network_conditions::client_connected)
    }

    #[wasm_bindgen]
    pub fn is_in_game(&mut self) -> bool {
        let mut query = self.0.world.query::<&Game>();
        query.iter(&self.0.world).count() > 0
    }

    #[wasm_bindgen]
    pub fn get_entity_count(&mut self) -> usize {
        let mut query = self.0.world.query::<Entity>();
        query.iter(&self.0.world).count()
    }

    #[wasm_bindgen]
    pub fn get_player_count(&mut self) -> usize {
        let mut query = self.0.world.query::<&Player>();
        query.iter(&self.0.world).count()
    }

    #[wasm_bindgen]
    pub fn setup_board(&mut self) {
        // let game_spawner = GameSpawner::new_game(GameBoard::WildChess, WinCondition::RoyalCapture);
        self.0.world.send_event(ConnectToServerEvent);
        // self.0.world.spawn((
        //     game_spawner.game,
        //     game_spawner.board,
        //     game_spawner.win_condition,
        // ));
    }

    #[wasm_bindgen]
    pub fn remove_board(&mut self) {
        // TODO: this queries for both game instance and board instance
        // what is the intended lifecycle of these components?
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
    // specifically, returns either "white" or "black"
    // TODO: be less "stringly typed" in a useful way?
    pub fn get_my_team(&mut self) -> Option<String> {
        let Some(client_id) = self
            .0
            .world
            .get_resource::<RepliconClient>()
            .and_then(|client| client.id())
        else {
            return None;
        };

        let mut query = self.0.world.query::<(&Player, &Team)>();
        let Some((_, team)) = query
            .iter(&self.0.world)
            .find(|(player, _)| player.id == client_id)
        else {
            return None;
        };

        Some(
            match team {
                Team::White => "white",
                Team::Black => "black",
            }
            .to_string(),
        )
    }

    #[wasm_bindgen]
    // specifically, returns either "white" or "black"
    // TODO: be less "stringly typed" in a useful way?
    pub fn get_piece_team(&mut self, square: String) -> Option<String> {
        let Ok(square): Result<Square, _> = square.as_str().try_into() else {
            return None;
        };

        let mut query = self.0.world.query::<(&Position, &Team)>();
        let Some((_, team)) = query
            .iter(&self.0.world)
            .find(|(position, _)| position.0 == square)
        else {
            return None;
        };

        Some(
            match team {
                Team::White => "white",
                Team::Black => "black",
            }
            .to_string(),
        )
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
        // TODO: not working after a first move is made
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

    // Vec should be size 2
    #[wasm_bindgen]
    pub fn get_last_move(&mut self) -> Option<Vec<WasmSquare>> {
        let mut query = self.0.world.query::<&LastMove>();
        let Ok(last_move) = query.get_single(&self.0.world) else {
            return None;
        };
        Some(vec![
            WasmSquare(last_move.0.movement.from),
            WasmSquare(last_move.0.movement.to),
        ])
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
        log(format!("{:?}", actions).as_str());

        let mut move_events = self.0.world.resource_mut::<Events<RequestTurnEvent>>();
        move_events.send(RequestTurnEvent::new(piece, action.clone()));
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

#[wasm_bindgen]
pub struct WasmGameRequest {
    pub(crate) variant: Option<GameRequestVariant>,
    pub(crate) clock: Option<GameRequestClock>,
}

#[wasm_bindgen]
impl WasmGameRequest {
    #[wasm_bindgen]
    pub fn new() -> Self {
        Self {
            variant: None,
            clock: None,
        }
    }

    #[wasm_bindgen]
    pub fn with_featured_game_one(mut self) -> Self {
        self.variant = Some(GameRequestVariant::FeaturedGameOne);
        self
    }

    #[wasm_bindgen]
    pub fn with_featured_game_two(mut self) -> Self {
        self.variant = Some(GameRequestVariant::FeaturedGameTwo);
        self
    }

    #[wasm_bindgen]
    pub fn with_featured_game_three(mut self) -> Self {
        self.variant = Some(GameRequestVariant::FeaturedGameThree);
        self
    }

    #[wasm_bindgen]
    pub fn with_wild_game(mut self) -> Self {
        self.variant = Some(GameRequestVariant::Wild);
        self
    }

    #[wasm_bindgen]
    pub fn with_classical_clock(mut self) -> Self {
        self.clock = Some(GameRequestClock::Classical);
        self
    }

    #[wasm_bindgen]
    pub fn with_rapid_clock(mut self) -> Self {
        self.clock = Some(GameRequestClock::Rapid);
        self
    }

    #[wasm_bindgen]
    pub fn with_blitz_clock(mut self) -> Self {
        self.clock = Some(GameRequestClock::Blitz);
        self
    }

    #[wasm_bindgen]
    pub fn with_bullet_clock(mut self) -> Self {
        self.clock = Some(GameRequestClock::Bullet);
        self
    }
}

fn get_orientation(
    client: Option<Res<RepliconClient>>,
    players: Query<(&Player, &Team)>,
) -> Orientation {
    if let Some((_, team)) = client
        .and_then(|client| client.id())
        .and_then(|client_id| players.iter().find(|(player, _)| player.id == client_id))
    {
        team.orientation()
    } else {
        Orientation::Up
    }
}
