use leptos::*;

use wildchess::games::chess::{board::Square, pieces::PieceDefinition};
use wildchess_web::{BoardState, BoardTargets, PlayerMessage};

use super::{grid::Grid, piece::Piece, square::Square};

#[component]
pub fn Board(
    #[prop(into)] state: Signal<BoardState>,
    #[prop(into)] targets: Signal<Option<BoardTargets>>,
    #[prop(into)] square_size: Signal<u16>,
    #[prop(into)] send_player_message: Signal<impl Fn(PlayerMessage) + 'static>,
) -> impl IntoView {
    let (selected_square, set_selected_square) = create_signal(None as Option<Square>);
    let (promotion_options, set_promotion_options) =
        create_signal(None as Option<Vec<PieceDefinition>>);

    let get_piece_on_square = move || {
        move |square: Square| {
            state
                .get()
                .pieces
                .0
                .get(&square)
                .map(|(id, team, mutation)| (square, *id, *team, mutation.clone()))
        }
    };

    let handle_selection = move |target: Square| {
        let selected_piece = selected_square
            .get()
            .and_then(|square| get_piece_on_square()(square));
        let action_for_target = targets
            .get()
            .and_then(|(targets)| targets.actions.get(&target).cloned());
        let target_piece = get_piece_on_square()(target);

        if let Some((origin, id, team, mutation)) = selected_piece {
            if let Some(action) = action_for_target {
                send_player_message.with(|send_player_message| {
                    send_player_message(PlayerMessage::RequestMove {
                        from: origin,
                        to: target,
                        promotion_index: None,
                    })
                });
                set_selected_square.set(None);
            } else if target == origin {
                set_selected_square.set(None);
            } else {
            }
        } else if let Some(piece) = get_piece_on_square()(target) {
            // set_selected_square.set(Some(square));
        }
    };

    view! {
        <div
            class="board"
            id="board"
            style=move || format!("height: {}px; width: {}px;", square_size.get() * state.get().size.1, square_size.get() * state.get().size.0)
        >
            <Grid
                rows=move || state.get().size.0
                columns=move || state.get().size.1
                children=move |row, column| {
                    let square = Square::new(column.into(), row.into());
                    let piece =  move || state.get().pieces.0.get(&square).cloned();
                    let icon = move || piece().and_then(move |piece| state.get().icons.0.get(&piece.0).cloned());
                    view! {
                        <Square
                            id=move || format!("square-{square}").into()
                            name=move || square.clone().to_string()
                            size=square_size
                            color=move || if selected_square.get().is_some_and(|selected_square| selected_square == square) {
                                "highlight".to_string()
                            } else if (row + column) % 2 == 0 {
                                "light-square".to_string()
                            } else {
                                "dark-square".to_string()
                            }
                            on_select=move || move || {
                                set_selected_square.set(Some(Square::new(column.into(), row.into())));
                            }
                        >
                            <Show
                                when=move || piece().is_some() && icon().is_some()
                                fallback=|| view! { <div /> }
                            >
                                <Piece
                                    piece=move || piece().unwrap()
                                    icon=move || icon().unwrap()
                                    square=move || Some(square.to_string())
                                    square_size=square_size
                                    hidden=|| false
                                />
                            </Show>
                        </Square>
                    }
                }
            />
        </div>
    }
}
