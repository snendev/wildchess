// #[cfg(not(target_arch = "wasm32"))]
// compile_error!("Compile this for wasm32 only!");

use std::time::Duration;

use wasm_bindgen::prelude::*;

use bevy_app::App;
use bevy_ecs::{
    prelude::{Entity, Events, Query, Res, With},
    system::RunSystemOnce,
    world::Command,
};

use games::{
    chess::{
        actions::Actions,
        board::Square,
        pieces::{Mutation, Orientation, PieceIdentity, Position, Royal},
        team::Team,
    },
    components::{
        Game, GameBoard, GameOver, GameRequestClock, GameRequestVariant, HasTurn, LastMove,
    },
    Clock, GameOpponent, GameplayPlugin, LeaveGameEvent, MatchmakingPlugin, RequestJoinGameEvent,
    RequestTurnEvent, RequireMutationEvent,
};
use replication::{
    replicon::{core::common_conditions as network_conditions, prelude::RepliconClient},
    replicon_renet2::renet2::RenetClient,
    Client, ClientCommand, ReplicationPlugin,
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
    pub fn new(ip: String, server_token: String) -> WasmApp {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let server_origin = ip;
        // let server_origin = option_env!("SERVER_ORIGIN").unwrap_or(&ip).to_string();
        let server_port = option_env!("SERVER_PORT").unwrap_or("7636").to_string();
        log(format!("Connecting to {server_origin}:{server_port}").as_str());

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
            ClientTransportPlugin {
                server_origin,
                server_port,
                wt_server_token: server_token,
            },
        ));
        app.add_plugins(wild_icons::PieceIconPlugin::new(get_orientation));

        WasmApp(app)
    }

    #[wasm_bindgen]
    pub fn request_online_game(&mut self, game_request: WasmGameRequest) {
        self.request_game(game_request, GameOpponent::Online);
    }

    #[wasm_bindgen]
    pub fn start_local_game(&mut self, game_request: WasmGameRequest) {
        ClientCommand::Disconnect.apply(self.0.world_mut());
        log("Client disconnected!");
        self.request_game(game_request, GameOpponent::Local);
    }

    fn request_game(&mut self, game_request: WasmGameRequest, opponent: GameOpponent) {
        self.0.world_mut().send_event(RequestJoinGameEvent {
            opponent,
            game: game_request.variant,
            clock: game_request.clock,
        });
    }

    #[wasm_bindgen]
    pub fn leave_game(&mut self) {
        let mut query = self.0.world_mut().query_filtered::<Entity, With<Game>>();
        for game in query.iter(self.0.world()).collect::<Vec<_>>() {
            self.0.world_mut().send_event(LeaveGameEvent { game });
        }
    }

    #[wasm_bindgen]
    pub fn is_connected(&mut self) -> bool {
        self.0
            .world_mut()
            .run_system_once(network_conditions::client_connected)
    }

    #[wasm_bindgen]
    pub fn is_in_game(&mut self) -> bool {
        let mut query = self.0.world_mut().query::<&Game>();
        query.iter(self.0.world()).count() > 0
    }

    #[wasm_bindgen]
    pub fn is_game_over(&mut self) -> Option<WasmGameover> {
        let mut query = self.0.world_mut().query::<(&Game, &GameOver)>();
        query
            .iter(self.0.world())
            .map(|(_, gameover)| WasmGameover {
                team: *gameover.winner(),
            })
            .next()
    }

    #[wasm_bindgen]
    pub fn get_entity_count(&mut self) -> usize {
        let mut query = self.0.world_mut().query::<Entity>();
        query.iter(self.0.world()).count()
    }

    #[wasm_bindgen]
    pub fn get_player_count(&mut self) -> usize {
        let mut query = self.0.world_mut().query::<&Client>();
        query.iter(self.0.world()).count()
    }

    #[wasm_bindgen]
    pub fn remove_board(&mut self) {
        // TODO: this queries for both game instance and board instance
        // what is the intended lifecycle of these components?
        let mut game_query = self
            .0
            .world_mut()
            .query_filtered::<Entity, With<GameBoard>>();
        for entity in game_query.iter(self.0.world()).collect::<Vec<_>>() {
            self.0.world_mut().entity_mut(entity).despawn();
        }
    }

    #[wasm_bindgen]
    pub fn check_game_state(&mut self) -> String {
        let mut query = self
            .0
            .world_mut()
            .query::<(&Position, &Team, &PieceIdentity)>();
        let mut buffer = String::from("");
        for (position, team, identity) in query.iter(self.0.world()) {
            buffer.push_str(format!("{:?} {:?}: {:?}\n", team, identity, position).as_str());
        }
        buffer
    }

    #[wasm_bindgen]
    pub fn is_my_turn(&mut self) -> bool {
        if self.0.world().get_resource::<RenetClient>().is_none() {
            return true;
        }
        let Some(client_id) = self
            .0
            .world()
            .get_resource::<RepliconClient>()
            .and_then(|client| client.id())
        else {
            return false;
        };
        let mut query = self
            .0
            .world_mut()
            .query_filtered::<&Client, With<HasTurn>>();
        query
            .iter(self.0.world())
            .find(|player| player.id == client_id)
            .is_some()
    }

    #[wasm_bindgen]
    // specifically, returns either "white" or "black"
    // TODO: be less "stringly typed" in a useful way?
    pub fn get_my_team(&mut self) -> Option<String> {
        let Some(client_id) = self
            .0
            .world()
            .get_resource::<RepliconClient>()
            .and_then(|client| client.id())
        else {
            return None;
        };

        let mut query = self.0.world_mut().query::<(&Client, &Team)>();
        let Some((_, team)) = query
            .iter(self.0.world())
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

        let mut query = self.0.world_mut().query::<(&Position, &Team)>();
        let Some((_, team)) = query
            .iter(self.0.world())
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
        let mut query = self
            .0
            .world_mut()
            .query::<(&Position, &Team, &PieceIdentity)>();
        query
            .iter(self.0.world())
            .map(|(position, team, identity)| {
                WasmPiecePosition(WasmPiece(*team, *identity), WasmSquare(position.0))
            })
            .collect()
    }

    #[wasm_bindgen]
    pub fn get_icons(&mut self) -> Vec<WasmIcon> {
        let mut query = self
            .0
            .world_mut()
            .query::<(&PieceIconSvg, &Team, &PieceIdentity)>();
        query
            .iter(self.0.world())
            .map(|(PieceIconSvg { source, .. }, team, identity)| WasmIcon {
                piece: WasmPiece(*team, *identity),
                svg_source: source.clone(),
            })
            .collect()
    }

    #[wasm_bindgen]
    pub fn get_clocks(&mut self) -> Vec<WasmClock> {
        let mut query = self.0.world_mut().query::<(&Team, &Clock)>();
        query
            .iter(self.0.world())
            .map(|(team, clock)| WasmClock {
                team: *team,
                clock: clock.remaining_time(),
            })
            .collect::<Vec<_>>()
    }

    #[wasm_bindgen]
    pub fn get_target_squares(&mut self, square: String) -> Option<Vec<WasmSquare>> {
        // TODO: not working after a first move is made
        let square: Square = square.as_str().try_into().expect("Invalid square!");
        let mut query = self.0.world_mut().query::<(&Position, &Actions)>();
        let (_, actions) = query
            .iter(self.0.world())
            .find(|(position, _)| position.0 == square)?;
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
        let mut query = self.0.world_mut().query::<&LastMove>();
        let Ok(last_move) = query.get_single(self.0.world()) else {
            return None;
        };
        Some(vec![
            WasmSquare(last_move.0.movement.from),
            WasmSquare(last_move.0.movement.to),
        ])
    }

    #[wasm_bindgen]
    pub fn trigger_move(
        &mut self,
        piece_square: String,
        target_square: String,
        promotion_index: Option<usize>,
    ) -> bool {
        let piece_square: Square = piece_square
            .as_str()
            .try_into()
            .expect(format!("a valid piece square: {piece_square}").as_str());
        let target_square: Square = target_square
            .as_str()
            .try_into()
            .expect(format!("a valid target square: {target_square}").as_str());
        // selectedPiece
        let mut query = self
            .0
            .world_mut()
            .query::<(Entity, &Position, &Actions, Option<&Mutation>)>();
        let Some((piece, _, actions, maybe_mutations)) = query
            .iter(self.0.world())
            .find(|(_, position, _, _)| position.0 == piece_square)
        else {
            return false;
        };
        let Some((_, action)) = actions
            .0
            .iter()
            .find(|(square, _)| **square == target_square)
        else {
            return false;
        };
        let action = action.clone();

        let promotion = maybe_mutations
            .zip(promotion_index)
            .and_then(|(mutation, index)| mutation.to_piece.get(index).cloned());

        let mut move_events = self
            .0
            .world_mut()
            .resource_mut::<Events<RequestTurnEvent>>();
        move_events.send(RequestTurnEvent {
            piece,
            action,
            promotion,
        });
        true
    }

    #[wasm_bindgen]
    pub fn select_promotion(&mut self, promotions: WasmPromotions, promotion_index: usize) -> bool {
        self.trigger_move(
            promotions.source.get_representation(),
            promotions.target.get_representation(),
            Some(promotion_index),
        )
    }

    #[wasm_bindgen]
    pub fn get_promotion_request(&mut self) -> Option<WasmPromotions> {
        let Some(mutation_request_events) = self
            .0
            .world()
            .get_resource::<Events<RequireMutationEvent>>()
        else {
            return None;
        };

        let mut reader = mutation_request_events.get_reader();
        // should only be one...
        let event = reader.read(&mutation_request_events).last();
        event.and_then(|event| {
            let Some(mutation) = self.0.world().get::<Mutation>(event.piece) else {
                return None;
            };
            let Some(team) = self.0.world().get::<Team>(event.piece) else {
                return None;
            };
            let maybe_royal = self.0.world().get::<Royal>(event.piece);
            let icons = mutation
                .to_piece
                .iter()
                .enumerate()
                .map(move |(index, option)| {
                    PieceIconSvg::new(
                        option.identity,
                        format!("promotion-{:?}", index),
                        option.behaviors.pattern.as_ref(),
                        option.behaviors.relay.as_ref(),
                        *team,
                        Orientation::Up,
                        maybe_royal.is_some(),
                    )
                    .source
                })
                .collect::<Vec<_>>();
            Some(WasmPromotions {
                icons,
                source: WasmSquare(event.action.movement.from),
                target: WasmSquare(event.action.movement.to),
            })
        })
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
pub struct WasmPromotions {
    icons: Vec<String>,
    source: WasmSquare,
    target: WasmSquare,
}

#[wasm_bindgen]
impl WasmPromotions {
    // Returns the piece name, like 'wP' for white pawn or 'bN' for black knight.
    #[wasm_bindgen]
    pub fn icons(&self) -> Vec<String> {
        self.icons.clone()
    }
}

#[wasm_bindgen]
pub struct WasmClock {
    team: Team,
    clock: Duration,
}

#[wasm_bindgen]
impl WasmClock {
    #[wasm_bindgen]
    pub fn get_team(&self) -> String {
        match self.team {
            Team::White => "white",
            Team::Black => "black",
        }
        .to_string()
    }

    #[wasm_bindgen]
    pub fn remaining_time(&self) -> String {
        let total_seconds = self.clock.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{}:{:02}", minutes, seconds)
    }
}

#[wasm_bindgen]
#[derive(Clone)]
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

#[wasm_bindgen]
pub struct WasmGameover {
    team: Team,
}

#[wasm_bindgen]
impl WasmGameover {
    #[wasm_bindgen]
    pub fn get_team(&self) -> String {
        match self.team {
            Team::White => "white",
            Team::Black => "black",
        }
        .to_string()
    }
}

fn get_orientation(
    client: Option<Res<RepliconClient>>,
    players: Query<(&Client, &Team)>,
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
