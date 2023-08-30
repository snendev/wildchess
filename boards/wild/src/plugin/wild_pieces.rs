use rand::{thread_rng, Rng};

use chess::pieces::Behavior;

use crate::pieces::PieceKind;

pub(crate) struct WildPieceSet {
    pub pieces: (Behavior, Behavior, Behavior, Behavior),
    pub pawn: Behavior,
    pub king: Behavior,
}

pub(crate) fn random_pieces() -> WildPieceSet {
    let mut rng = thread_rng();
    let max_value: u32 = rng.gen_range(50..80);
    let mut current_value: u32 = 0;

    // pieces
    let ah = PieceKind::generate_piece(max_value, &mut current_value);
    let bg = PieceKind::generate_piece(max_value, &mut current_value);
    let cf = PieceKind::generate_piece(max_value, &mut current_value);
    let d = PieceKind::generate_piece(max_value, &mut current_value);
    // pawns
    let pawn = PieceKind::generate_pawn();
    // king
    let king = PieceKind::generate_king();

    WildPieceSet {
        pieces: (ah, bg, cf, d),
        pawn,
        king,
    }
}
