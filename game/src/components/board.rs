// The board is the structure that defines the game configuration

use rand::{thread_rng, Rng};

use bevy::prelude::Component;

use crate::{
    components::{Behavior, PieceConfiguration, PieceKind, Promotable, StartPosition},
    File, LocalSquare, Rank,
};

#[derive(Clone, Debug, Hash)]
pub struct BoardPieces(pub Vec<(PieceConfiguration, Vec<StartPosition>)>);

impl From<Vec<(PieceConfiguration, Vec<StartPosition>)>> for BoardPieces {
    fn from(value: Vec<(PieceConfiguration, Vec<StartPosition>)>) -> Self {
        BoardPieces(value)
    }
}

#[derive(Clone, Component, Debug, Hash)]
pub struct Board {
    pub pieces: BoardPieces,
    // TODO this not used yet
    pub size: (u8, u8),
    // id: BoardId,
    // wtf else does this need
}

impl Board {
    pub fn from_pieces(pieces: BoardPieces) -> Self {
        Board {
            pieces,
            size: (8, 8),
        }
    }

    pub fn wild_configuration() -> Self {
        Board::from_pieces(BoardPieces(random_chess_configurations()))
    }
}

fn random_chess_configurations() -> Vec<(PieceConfiguration, Vec<StartPosition>)> {
    let mut rng = thread_rng();
    let max_value: u32 = rng.gen_range(50..80);
    let mut current_value: u32 = 0;

    // pieces
    let ah = PieceKind::generate_piece(max_value, &mut current_value);
    let bg = PieceKind::generate_piece(max_value, &mut current_value);
    let cf = PieceKind::generate_piece(max_value, &mut current_value);
    let d = PieceKind::generate_piece(max_value, &mut current_value);
    // pawns
    let pawn_promotion_options = vec![ah.clone(), bg.clone(), cf.clone(), d.clone()];
    let pawn = PieceKind::generate_pawn(Promotable {
        local_rank: Rank::Eight,
        behaviors: pawn_promotion_options.clone(),
    });
    // king
    let king = PieceKind::generate_king();

    let make_piece_config = |behavior: Behavior| PieceConfiguration {
        kind: PieceKind::Piece,
        behavior,
        promotable: None,
    };
    let rank_one_square = |file: File| StartPosition(LocalSquare::new(Rank::One, file));

    vec![
        // pieces
        (
            make_piece_config(ah),
            vec![rank_one_square(File::A), rank_one_square(File::H)],
        ),
        (
            make_piece_config(bg),
            vec![rank_one_square(File::B), rank_one_square(File::G)],
        ),
        (
            make_piece_config(cf),
            vec![rank_one_square(File::C), rank_one_square(File::F)],
        ),
        (make_piece_config(d), vec![rank_one_square(File::D)]),
        // pawns
        (
            pawn.clone(),
            File::all()
                .map(|file| StartPosition(LocalSquare::new(Rank::Two, file)))
                .collect(),
        ),
        // king
        (king, vec![rank_one_square(File::E)]),
    ]
}
