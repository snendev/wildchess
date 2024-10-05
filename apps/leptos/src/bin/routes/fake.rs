use leptos::*;

use wildchess::games::chess::{
    behavior::PatternBehavior,
    board::{File, Rank, Square},
    pattern::Pattern,
    pieces::{Orientation, PieceIdentity},
    team::Team,
};
use wildchess::wild_icons::PieceIconSvg;
use wildchess_web::{BoardState, PieceIconMap, PieceMap};

#[component]
pub fn FakeBoard() -> impl IntoView {
    use super::game::Board;
    view! {
        <Board
            state=|| {
                let pieces = fallback_pieces();
                let icons = fallback_icons();
                BoardState { size: (8, 8), pieces, icons, ..Default::default() }
            }
            targets=|| None
            send_player_message=move || |_| {}
            square_size=|| 80
        />
    }
}

fn fallback_pieces() -> PieceMap {
    let mut piece_map = PieceMap::default();
    piece_map.0.insert(
        Square::new(File::A, Rank::ONE),
        (PieceIdentity::King, Team::White, None),
    );
    piece_map.0.insert(
        Square::new(File::H, Rank::EIGHT),
        (PieceIdentity::Queen, Team::Black, None),
    );
    piece_map
}

fn fallback_icons() -> PieceIconMap {
    let mut piece_icon_map = PieceIconMap::default();
    let king_behavior = PatternBehavior::default()
        .with_pattern(Pattern::radial().range(1).captures_by_displacement());
    let queen_behavior =
        PatternBehavior::default().with_pattern(Pattern::radial().captures_by_displacement());
    let w_k_icon = PieceIconSvg::new(
        PieceIdentity::King,
        "",
        Some(&king_behavior),
        None,
        Team::White,
        Orientation::Up,
        true,
    );
    let b_q_icon = PieceIconSvg::new(
        PieceIdentity::Queen,
        "",
        Some(&queen_behavior),
        None,
        Team::Black,
        Orientation::Up,
        false,
    );
    piece_icon_map.0.insert(PieceIdentity::King, w_k_icon);
    piece_icon_map.0.insert(PieceIdentity::Queen, b_q_icon);
    piece_icon_map
}
