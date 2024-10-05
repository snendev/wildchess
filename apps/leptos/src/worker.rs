use gloo_worker::{HandlerId, Worker, WorkerScope};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use wildchess::bevy::ecs::entity::Entity;
// use client::{
//     bevy_replicon::prelude::{RepliconClient, RepliconClientStatus},
//     ClientPlugin,
// };
use wildchess::bevy::app::App;
use wildchess::bevy::ecs::event::Events;
use wildchess::bevy::ecs::world::World;
use wildchess::bevy::utils::{HashMap, HashSet};
use wildchess::bevy_replicon::prelude::RepliconClient;
use wildchess::bevy_replicon::prelude::RepliconClientStatus;
use wildchess::games::chess::actions::Actions;
use wildchess::games::chess::pieces::{Mutation, MutationCondition, Position};
use wildchess::games::chess::team::Team;
use wildchess::games::components::Client;
use wildchess::games::components::InGame;
use wildchess::games::RequestTurnEvent;

use crate::{
    BoardState, BoardTargets, PlayerMessage, WorkerMessage, SERVER_DEFAULT_IP,
    SERVER_DEFAULT_ORIGIN, SERVER_DEFAULT_PORT, SERVER_DEFAULT_TOKENS_PORT, SERVER_IP,
    SERVER_ORIGIN, SERVER_PORT, SERVER_TOKENS_PORT,
};

// Use this to enable console logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: String);
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> u64;
    fn clearInterval(token: u64);
}

pub struct BevyWorker {
    game: Option<App>,
    subscriptions: HashSet<HandlerId>,
    _trigger_update: Closure<dyn FnMut()>,
    interval: Interval,
}

impl PartialEq for BevyWorker {
    fn eq(&self, other: &Self) -> bool {
        self.interval.0 == other.interval.0
    }
}

impl Worker for BevyWorker {
    type Input = PlayerMessage;
    type Output = WorkerMessage;
    type Message = WorkerUpdateMessage;

    fn create(scope: &WorkerScope<Self>) -> Self {
        scope
            .send_future(async { WorkerUpdateMessage::Token(fetch_server_token().await.unwrap()) });
        let scope_clone = scope.clone();
        let trigger_update = Closure::new(move || {
            scope_clone.send_message(WorkerUpdateMessage::Update);
        });
        let interval = setInterval(&trigger_update, 10);
        Self {
            game: None,
            subscriptions: HashSet::default(),
            interval: Interval(interval),
            _trigger_update: trigger_update,
        }
    }

    fn connected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        self.subscriptions.insert(id);
    }

    fn update(&mut self, scope: &WorkerScope<Self>, message: Self::Message) {
        if let Some(app) = self.game.as_mut() {
            let WorkerUpdateMessage::Update = message else {
                return;
            };
            app.update();

            let Some((_, my_side)) = get_my_player(app.world_mut()) else {
                return;
            };
            // let events = app.world().resource::<Events<ActiveGameUpdate>>();
            // let mut reader = events.get_reader();
            // if let Some(update) = reader.read(events).last() {
            //     for id in &self.subscriptions {
            //         scope.respond(
            //             *id,
            //             WorkerMessage::State(BoardState {
            //                 orientation,
            //                 pieces,
            //                 icons,
            //             }),
            //         );
            //     }
            // }
        } else if let WorkerUpdateMessage::Token(token) = message {
            let app = build_app(token);
            self.game = Some(app);
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, message: Self::Input, handler_id: HandlerId) {
        let Some(app) = self.game.as_mut() else {
            #[cfg(feature = "log")]
            log(format!(
                "Discarding message received before app is ready: {:?}",
                message
            ));
            return;
        };
        let replicon_client = app.world().resource::<RepliconClient>();
        let RepliconClientStatus::Connected {
            client_id: Some(my_client_id),
        } = replicon_client.status()
        else {
            #[cfg(feature = "log")]
            log(format!(
                "Discarding message received before client is connected: {:?}",
                message
            ));
            return;
        };
        #[cfg(feature = "log")]
        log(format!("Message received! {:?}", message));

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

struct Interval(u64);

impl Drop for Interval {
    fn drop(&mut self) {
        clearInterval(self.0);
    }
}

fn build_app(server_token: String) -> App {
    let mut app = App::new();
    let server_origin = SERVER_IP.unwrap_or(SERVER_DEFAULT_IP).to_string();
    let server_port = SERVER_PORT.unwrap_or(SERVER_DEFAULT_PORT).to_string();

    // app.add_plugins(WildchessPlugins);
    // // app.add_plugins(ClientPlugin {
    // //     server_origin,
    // //     server_port,
    // //     server_token,
    // // });
    app.update();
    app.update();
    app
}

fn get_my_player(world: &mut World) -> Option<(Entity, Team)> {
    let replicon_client = world.resource::<RepliconClient>();
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

async fn fetch_server_token() -> Result<String, JsValue> {
    let server_origin = SERVER_ORIGIN.unwrap_or(SERVER_DEFAULT_ORIGIN);
    let server_token_port = SERVER_TOKENS_PORT.unwrap_or(SERVER_DEFAULT_TOKENS_PORT);
    let server_url = format!("{server_origin}:{server_token_port}");
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(&server_url, &opts)?;

    let response = JsFuture::from(fetch(&request)).await?;

    assert!(response.is_instance_of::<Response>());
    let response: Response = response.dyn_into().unwrap();
    let text = JsFuture::from(response.text()?).await?;
    log(text.as_string().unwrap());
    Ok(text.as_string().unwrap())
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
                #[cfg(feature = "log")]
                error(format!("Warning! Piece not found at square {piece_square}"));
                return WorkerMessage::Targets(None);
            };
            let game = in_game.0;
            let promotion = maybe_mutations
                .zip(promotion_index)
                .and_then(|(mutation, index)| mutation.to_piece.get(index).cloned());

            // get the action being taken
            let Some((_, action)) = actions.0.iter().find(|(square, _)| **square == to) else {
                #[cfg(feature = "log")]
                error(format!(
                    "Warning! Action not found for target {piece_square}"
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
                #[cfg(feature = "log")]
                error(format!("No action not found for target {square}."));
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
