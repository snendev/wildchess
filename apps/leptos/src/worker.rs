use gloo_worker::{HandlerId, Worker, WorkerScope};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Response, WorkerGlobalScope};

use wildchess::bevy::log::LogPlugin;
use wildchess::bevy::prelude::{App, Entity, With, World};
use wildchess::bevy::utils::{HashMap, HashSet};
use wildchess::bevy_replicon::prelude::RepliconClient;
use wildchess::bevy_replicon::prelude::RepliconClientStatus;
use wildchess::client::ConnectToServer;
use wildchess::games::chess::actions::Actions;
use wildchess::games::chess::pieces::{Mutation, Position};
use wildchess::games::chess::team::Team;
use wildchess::games::components::Client;
use wildchess::games::components::InGame;
use wildchess::games::RequestTurnEvent;
use wildchess::{Active, BoardState, WildchessPlugins};

use crate::{debug, error, log, warn, BoardTargets, PlayerMessage, WorkerMessage};

pub struct BevyWorker {
    game: Option<App>,
    subscriptions: HashSet<HandlerId>,
    _update_interval: Interval,
}

impl BevyWorker {
    fn update_bevy_state(&mut self) -> Option<WorkerMessage> {
        let Some(app) = self.game.as_mut() else {
            debug("No game - skipping update");
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
        scope.send_future(async {
            let token = fetch_server_token().await.unwrap();
            WorkerUpdateMessage::Token(token.as_string().unwrap())
        });
        let scope = scope.clone();
        let update_interval = Interval::new(10, move || {
            scope.send_message(WorkerUpdateMessage::Update);
        })
        .unwrap();
        Self {
            game: None,
            subscriptions: HashSet::default(),
            _update_interval: update_interval,
        }
    }

    fn connected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        self.subscriptions.insert(id);
    }

    fn update(&mut self, scope: &WorkerScope<Self>, message: Self::Message) {
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
    }

    fn received(&mut self, scope: &WorkerScope<Self>, message: Self::Input, handler_id: HandlerId) {
        let Some(app) = self.game.as_mut() else {
            warn(&format!(
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
            debug(&format!(
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

fn build_app(server_token: String) -> App {
    use wildchess::bevy::MinimalPlugins;

    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(LogPlugin::default());
    app.add_plugins(WildchessPlugins);
    app.world_mut().trigger(ConnectToServer {
        token: server_token,
    });
    app.update();
    app.update();
    app
}

fn get_my_player(world: &mut World) -> Option<(Entity, Team)> {
    let Some(replicon_client) = world.get_resource::<RepliconClient>() else {
        warn("no replicon client found");
        return None;
    };
    let status = replicon_client.status();
    let RepliconClientStatus::Connected {
        client_id: Some(my_client_id),
    } = status
    else {
        debug(&format!("client not yet connected; status: {:?}", status));
        return None;
    };
    let mut query = world.query::<(Entity, &Team, &Client)>();
    let (my_player, my_side, _) = query
        .iter(world)
        .find(|(_, _, client)| client.id == my_client_id)?;
    Some((my_player, *my_side))
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

pub struct Interval {
    _closure: Closure<dyn FnMut()>,
    handle: i32,
}

impl Interval {
    pub fn new<F: FnMut() + 'static>(millis: i32, f: F) -> Result<Interval, JsValue> {
        let closure: Closure<dyn FnMut()> = Closure::new(f);

        let global = js_sys::global();
        let global: WorkerGlobalScope = global.dyn_into()?;
        let handle = global.set_interval_with_callback_and_timeout_and_arguments_0(
            closure
                .as_ref()
                .dyn_ref()
                .ok_or(JsValue::from_str("failed to cast closure"))?,
            millis,
        )?;

        Ok(Interval {
            _closure: closure,
            handle,
        })
    }
}

// When the Interval is destroyed, clear its `setInterval` timer.
impl Drop for Interval {
    fn drop(&mut self) {
        let global = js_sys::global();
        let global: WorkerGlobalScope = global.dyn_into().unwrap();
        global.clear_interval_with_handle(self.handle);
    }
}

fn server_token_request() -> web_sys::Request {
    use wildchess::network_constants::{
        SERVER_DEFAULT_ORIGIN, SERVER_DEFAULT_TOKENS_PORT, SERVER_ORIGIN, SERVER_TOKENS_PORT,
    };

    let server_origin = SERVER_ORIGIN.unwrap_or(SERVER_DEFAULT_ORIGIN);
    let server_token_port = SERVER_TOKENS_PORT.unwrap_or(SERVER_DEFAULT_TOKENS_PORT);
    let server_url = format!("{server_origin}:{server_token_port}");

    web_sys::Request::new_with_str(&server_url).expect("Request to have a valid URL")
}

async fn fetch_server_token() -> Result<JsValue, JsValue> {
    let request = server_token_request();
    let global = js_sys::global();
    let global: WorkerGlobalScope = global.dyn_into()?;
    let response = JsFuture::from(global.fetch_with_request(&request)).await?;
    let response: Response = response.dyn_into()?;
    let text_promise = response.text()?;
    JsFuture::from(text_promise).await
}
