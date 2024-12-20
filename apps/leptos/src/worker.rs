use gloo_worker::{HandlerId, Worker, WorkerScope};
use js_sys::Promise;
use leptos::set_timeout;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use wildchess::bevy::app::App;
use wildchess::bevy::ecs::entity::Entity;
use wildchess::bevy::ecs::world::World;
use wildchess::bevy::prelude::With;
use wildchess::bevy::utils::{HashMap, HashSet};
use wildchess::bevy_replicon::prelude::RepliconClient;
use wildchess::bevy_replicon::prelude::RepliconClientStatus;
use wildchess::games::chess::actions::Actions;
use wildchess::games::chess::pieces::{Mutation, Position};
use wildchess::games::chess::team::Team;
use wildchess::games::components::Client;
use wildchess::games::components::InGame;
use wildchess::games::RequestTurnEvent;
use wildchess::{Active, BoardState};

use crate::{
    BoardTargets, PlayerMessage, WorkerMessage, SERVER_DEFAULT_IP, SERVER_DEFAULT_ORIGIN,
    SERVER_DEFAULT_PORT, SERVER_DEFAULT_TOKENS_PORT, SERVER_IP, SERVER_ORIGIN, SERVER_PORT,
    SERVER_TOKENS_PORT,
};

// Use this to enable console logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

pub struct BevyWorker {
    game: Option<App>,
    subscriptions: HashSet<HandlerId>,
}

impl BevyWorker {
    fn update_bevy_state(&mut self) -> Option<WorkerMessage> {
        let Some(app) = self.game.as_mut() else {
            return None;
        };

        app.update();

        let Some((_, my_team)) = get_my_player(app.world_mut()) else {
            return None;
        };

        let mut query = app
            .world_mut()
            .query_filtered::<&BoardState, With<Active>>();
        let state = query.single(app.world());

        Some(WorkerMessage::State {
            state: state.clone(),
            my_team,
        })
    }
}

impl Worker for BevyWorker {
    type Input = PlayerMessage;
    type Output = WorkerMessage;
    type Message = WorkerUpdateMessage;

    fn create(scope: &WorkerScope<Self>) -> Self {
        log("create");
        scope.send_message(
            WorkerUpdateMessage::fetch_token().expect("to fetch a token successfully"),
        );

        Self {
            game: None,
            subscriptions: HashSet::default(),
        }
    }

    fn connected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        log("connected");
        self.subscriptions.insert(id);
    }

    fn update(&mut self, scope: &WorkerScope<Self>, message: Self::Message) {
        log("update");

        match message {
            WorkerUpdateMessage::Token(token) => {
                let app = build_app(token);
                self.game = Some(app);
            }
            WorkerUpdateMessage::Update => {
                if let Some(response) = self.update_bevy_state() {
                    for id in &self.subscriptions {
                        scope.respond(*id, response.clone());
                    }
                }
            }
        }
        scope.send_message(WorkerUpdateMessage::Update);
    }

    fn received(&mut self, scope: &WorkerScope<Self>, message: Self::Input, handler_id: HandlerId) {
        log("received");
        let Some(app) = self.game.as_mut() else {
            log(&format!(
                "Discarding message received before app is ready: {:?}",
                message
            ));
            return;
        };
        let replicon_client = app
            .world()
            .get_resource::<RepliconClient>()
            .expect("RepliconPlugins to be added to game");
        let RepliconClientStatus::Connected {
            client_id: Some(my_client_id),
        } = replicon_client.status()
        else {
            log(&format!(
                "Discarding message received before client is connected: {:?}",
                message
            ));
            return;
        };
        log(&format!(
            "Client {} received a message! {:?}",
            my_client_id.get(),
            message
        ));

        // todo: where are we checking who the player is?
        let response = handle_message(app, message);
        scope.respond(handler_id, response);

        app.update();
    }
}

pub enum WorkerUpdateMessage {
    Token(String),
    Update,
}

impl WorkerUpdateMessage {
    fn fetch_token() -> Result<Self, JsValue> {
        let server_origin = SERVER_ORIGIN.unwrap_or(SERVER_DEFAULT_ORIGIN);
        let server_token_port = SERVER_TOKENS_PORT.unwrap_or(SERVER_DEFAULT_TOKENS_PORT);
        let server_url = format!("{server_origin}:{server_token_port}");
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);
        let request = Request::new_with_str_and_init(&server_url, &opts)?;

        let response = fetch(&request).then(&mut Closure::new(|js_value: JsValue| {
            let response: Response = js_value.dyn_into().unwrap();
            let text: String = response.text().unwrap();
            text.as_string()
        }));

        assert!(response.is_instance_of::<Response>());
        let response: Response = response.dyn_into().unwrap();

        log("3");
        let text =
            futures_lite::future::block_on(async { JsFuture::from(response.text()?).await })?;
        log("4");
        let token = text.as_string().expect("Server token to be a string");

        log(&token);
        Ok(WorkerUpdateMessage::Token(token))
    }
}

fn build_app(server_token: String) -> App {
    let mut app = App::new();
    log("Building app!");
    app.add_plugins(wildchess::WildchessPlugins::as_client(
        SERVER_IP.unwrap_or(SERVER_DEFAULT_IP).to_string(),
        SERVER_PORT.unwrap_or(SERVER_DEFAULT_PORT).to_string(),
        server_token,
    ));
    app.update();
    app.update();
    app
}

fn get_my_player(world: &mut World) -> Option<(Entity, Team)> {
    let Some(replicon_client) = world.get_resource::<RepliconClient>() else {
        return None;
    };
    let RepliconClientStatus::Connected {
        client_id: Some(my_client_id),
    } = replicon_client.status()
    else {
        return None;
    };
    let mut query = world.query::<(Entity, &Team, &Client)>();
    let (my_player, my_side, _) = query
        .iter(world)
        .find(|(_, _, client)| client.id == my_client_id)?;
    Some((my_player, *my_side))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn fetch(input: &Request) -> Promise;
}

fn handle_message(app: &mut App, message: PlayerMessage) -> WorkerMessage {
    match message {
        PlayerMessage::RequestMove {
            from,
            to,
            promotion_index,
        } => {
            let mut query =
                app.world_mut()
                    .query::<(Entity, &Position, &Actions, Option<&Mutation>, &InGame)>();

            // get the selected piece data
            let Some((piece, _, actions, maybe_mutations, in_game)) = query
                .iter(app.world())
                .find(|(_, position, _, _, _)| position.0 == from)
            else {
                error(&format!("Warning! Piece not found at square {from}"));
                return WorkerMessage::Targets(None);
            };
            let game = in_game.0;
            let promotion = maybe_mutations
                .zip(promotion_index)
                .and_then(|(mutation, index)| mutation.to_piece.get(index).cloned());

            // get the action being taken
            let Some((_, action)) = actions.0.iter().find(|(square, _)| **square == to) else {
                error(&format!(
                    "Warning! Action not found for piece on square {from}"
                ));
                return WorkerMessage::Targets(None);
            };
            let action = action.clone();

            // request a turn and be optimistic
            app.world_mut().send_event(RequestTurnEvent {
                game,
                piece,
                action,
                promotion,
            });

            WorkerMessage::Targets(None)
        }
        PlayerMessage::SelectPiece { square } => {
            let mut query = app
                .world_mut()
                .query::<(&Position, &Actions, Option<&Mutation>)>();
            let Some((_, actions, mutations)) = query
                .iter(app.world())
                .find(|(position, _, _)| position.0 == square)
            else {
                error(&format!("No action not found for target {square}."));
                return WorkerMessage::Targets(None);
            };

            let actions = if let Some(mutation) = mutations {
                actions
                    .0
                    .iter()
                    .flat_map(|(square, action)| {
                        mutation
                            .to_piece
                            .iter()
                            .map(|piece| (*square, (action.clone(), Some(piece.clone()))))
                    })
                    .collect::<HashMap<_, _>>()
            } else {
                actions
                    .0
                    .iter()
                    .flat_map(|(square, action)| std::iter::once((*square, (action.clone(), None))))
                    .collect::<HashMap<_, _>>()
            };

            WorkerMessage::Targets(Some(BoardTargets {
                origin: square,
                actions,
            }))
        }
        PlayerMessage::OfferDraw => todo!(),
        PlayerMessage::AcceptDraw => todo!(),
        PlayerMessage::Resign => todo!(),
    }
}
