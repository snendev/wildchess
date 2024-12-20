use gloo_worker::{Spawnable, WorkerBridge};
use leptos::*;

use wildchess::{games::chess::team::Team, BoardState};
use wildchess_web::{BevyWorker, BoardTargets, PlayerMessage, WorkerMessage};

mod board;
pub use board::*;
mod grid;
mod piece;
mod square;

const BEVY_WORKER: std::cell::OnceCell<WorkerBridge<BevyWorker>> = std::cell::OnceCell::new();

#[component]
pub fn Game() -> impl IntoView {
    let (board_state, set_board_state) = create_signal(None as Option<BoardState>);
    let (my_team, set_my_team) = create_signal(Team::White);
    let (board_targets, set_board_targets) = create_signal(None as Option<BoardTargets>);
    wildchess_web::log("Game!".to_string());

    create_effect(move |_| {
        wildchess_web::log("Spawning worker!".to_string());
        let message_callback = move |message: WorkerMessage| {
            wildchess_web::log(format!("Update from Bevy app: {:?}", message));
            wildchess_web::log("Setting state...".to_string());
            match message {
                WorkerMessage::State { state, my_team } => {
                    set_board_state.set(Some(state));
                    set_my_team.set(my_team);
                    set_board_targets.set(None);
                }
                WorkerMessage::Targets(board_targets) => {
                    set_board_targets.set(board_targets);
                }
            }
        };
        BEVY_WORKER
            .set(
                BevyWorker::spawner()
                    .callback(message_callback)
                    .spawn("/worker.js"),
            )
            .unwrap();
    });

    view! {
        {move || match board_state.get() {
            Some(_) => {
                wildchess_web::log("Spawning board!".to_string());
                let handle_player_message = |message: PlayerMessage| {
                    BEVY_WORKER
                        .get()
                        .expect("Bevy worker to be initialized before sending messages!")
                        .send(message);
                };
                view! {
                    <Board
                        state=move || board_state.get().unwrap()
                        my_team=my_team
                        targets=board_targets
                        send_player_message=move || handle_player_message
                        square_size=|| 80
                    />
                }
            }
            .into_view(),
            None => {
                wildchess_web::log("Loading game app!".to_string());
                view! {
                    <h2>Loading game...</h2>
                }
            }
            .into_view(),
        }}
    }
}
