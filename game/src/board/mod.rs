use bevy::prelude::Commands;

use rand::{thread_rng, Rng};

use crate::{
    pieces::{GamePieces, PieceBundle, PieceConfiguration, PieceKind},
    File, Square, Team,
};

fn random_chess_configurations() -> Vec<PieceConfiguration> {
    let mut rng = thread_rng();
    let max_value: u32 = rng.gen_range(50..80);
    let mut current_value: u32 = 0;

    let mut configs = Vec::new();

    for team in [Team::White, Team::Black] {
        let ah = PieceKind::generate_piece(max_value, &mut current_value);
        let bg = PieceKind::generate_piece(max_value, &mut current_value);
        let cf = PieceKind::generate_piece(max_value, &mut current_value);
        for file in [File::A, File::H] {
            configs.push(PieceConfiguration {
                kind: PieceKind::SquareAH,
                behavior: ah.clone(),
                starting_square: Square::piece(file, team),
            });
        }
        for file in [File::B, File::G] {
            configs.push(PieceConfiguration {
                kind: PieceKind::SquareBG,
                behavior: bg.clone(),
                starting_square: Square::piece(file, team),
            });
        }
        for file in [File::C, File::F] {
            configs.push(PieceConfiguration {
                kind: PieceKind::SquareCF,
                behavior: cf.clone(),
                starting_square: Square::piece(file, team),
            });
        }
        configs.push(PieceConfiguration {
            kind: PieceKind::SquareD,
            behavior: PieceKind::generate_piece(max_value, &mut current_value),
            starting_square: Square::piece(File::D, team),
        });
        for file in File::all() {
            configs.push(PieceKind::generate_pawn(Square::pawn(file, team)));
        }
        configs.push(PieceKind::generate_king(Square::piece(File::E, team)));
    }
    configs
}

pub fn wild_board(mut commands: Commands) {
    let pieces = random_chess_configurations();

    commands.insert_resource(GamePieces(pieces.clone()));

    for team in vec![Team::White, Team::Black] {
        // the pieces are placed on the board symmetrically outside->in,
        // such that the fourth behavior is only spawned once
        commands.spawn_batch(
            pieces
                .clone()
                .into_iter()
                .map(move |config| PieceBundle::from_configuration(config, team)),
        );
    }
}
