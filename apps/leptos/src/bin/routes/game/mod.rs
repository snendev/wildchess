use gloo_worker::Spawnable;
use leptos::*;

use wildchess::{games::chess::team::Team, BoardState};
use wildchess_web::{BevyWorker, BoardTargets, PlayerMessage, WorkerMessage};

mod board;
pub use board::*;
mod grid;
mod piece;
mod square;

#[component]
pub fn Game() -> impl IntoView {
    let (board_state, set_board_state) = create_signal(None as Option<BoardState>);
    let (my_team, set_my_team) = create_signal(Team::White);
    let (board_targets, set_board_targets) = create_signal(None as Option<BoardTargets>);
    wildchess_web::log("Game!");

    let bridge = create_memo(move |_| {
        wildchess_web::log("Fetching server token and spawning worker...");

        wildchess_web::log("Spawning worker!");
        let message_callback = move |message: WorkerMessage| {
            wildchess_web::log(&format!("Update from Bevy app: {:?}", message));
            wildchess_web::log("Setting state...");
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
        BevyWorker::spawner()
            .callback(message_callback)
            .spawn("/worker.js")
    });

    view! {
        {move || match board_state.get() {
            Some(_) => {
                wildchess_web::log("Spawning board!");
                let handle_player_message = move |message: PlayerMessage| {
                    bridge.with(|bridge| bridge.send(message));
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
                wildchess_web::log("Loading game app!");
                view! {
                    <h2>Loading game...</h2>
                    {bridge.with(|_| {})}
                }
            }
            .into_view(),
        }}
    }
}
