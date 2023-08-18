use rand::{thread_rng, Rng};

use bevy::prelude::{App, Commands, Plugin, Startup};

use crate::{
    components::{Behavior, PieceKind, Promotable, StartPosition},
    File, GamePieces, LocalSquare, PieceConfiguration, Rank,
};

fn random_chess_configurations() -> Vec<(PieceConfiguration, Vec<StartPosition>)> {
    let mut rng = thread_rng();
    let max_value: u32 = rng.gen_range(50..80);
    let mut current_value: u32 = 0;

    // pieces
    let ah = PieceKind::generate_piece(max_value, &mut current_value);
    let bg = PieceKind::generate_piece(max_value, &mut current_value);
    let cf = PieceKind::generate_piece(max_value, &mut current_value);
    let d = PieceKind::generate_piece(max_value, &mut current_value);
    let pawn_promotion_options = vec![ah.clone(), bg.clone(), cf.clone(), d.clone()];
    let pawn = PieceKind::generate_pawn(Promotable {
        local_rank: Rank::Eight,
        behaviors: pawn_promotion_options.clone(),
    });
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

fn add_wild_pieces(mut commands: Commands) {
    let pieces = random_chess_configurations();
    commands.insert_resource(GamePieces(pieces.clone()));
}

pub struct WildBoardPlugin;

impl Plugin for WildBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_wild_pieces);
    }
}
