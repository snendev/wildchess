use gloo_worker::Spawnable;
use leptos::*;

use wildchess_web::{BevyWorker, BoardState, BoardTargets, PlayerMessage, WorkerMessage};

mod board;
pub use board::*;
mod grid;
mod piece;
mod square;

#[component]
pub fn Game() -> impl IntoView {
    let (board_state, set_board_state) = create_signal(None as Option<BoardState>);
    let (board_targets, set_board_targets) = create_signal(None as Option<BoardTargets>);

    let worker = create_memo(move |_| {
        let message_callback = move |message: WorkerMessage| {
            #[cfg(feature = "log")]
            wildchess_web::log(format!("Update from Bevy app: {:?}", message));
            #[cfg(feature = "log")]
            wildchess_web::log("Setting state...".to_string());
            match message {
                WorkerMessage::State(board_state) => {
                    set_board_state.set(Some(board_state));
                    set_board_targets.set(None);
                }
                WorkerMessage::Targets(board_targets) => {
                    set_board_targets.set(board_targets);
                }
            }
        };
        BevyWorker::spawner()
            .callback(message_callback)
            .spawn("./worker.js")
    });

    let handle_player_message = move |message: PlayerMessage| {
        worker.with(|worker| {
            worker.send(message);
        });
    };

    match board_state.get() {
        Some(_) => view! {
            <Board
                state=move || board_state.get().unwrap()
                targets=board_targets
                send_player_message=move || handle_player_message
                square_size=|| 80
            />
        }
        .into_view(),
        None => view! {
            <h2>Loading game...</h2>
        }
        .into_view(),
    }
}
